use crate::message::{BusMessage, OpaqueBusMessage, Signal};
use crate::port::{InputPort, OutputPort};
use crate::bus::Bus;
use crate::component::Component;
use std::collections::{HashMap};
use anyhow::{Result};
use chrono::{DateTime, Utc, NaiveTime, TimeDelta};

const SIM_TIME_BEGIN: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

/// A small context object that individual components
/// use to read/write inputs/outputs that they 
/// had previously defined in their component definition 
/// ONLY. Fields are private, accessible with the right
/// getters
pub struct StepContext<'a> {

    /// Real wall clock time of this simulation step
    wall_time: DateTime<Utc>,

    /// simulation time
    sim_time: NaiveTime,

    /// real time delta since last tick
    wall_dt: TimeDelta,

    /// simulation time delta since last tick
    sim_dt: TimeDelta,

    /// Tick number, monotonically increasing
    tick_number: u64,

    /// The inputs field should be populated
    /// with references to real messages 
    /// currently existing on bus channels
    inputs: HashMap<String, &'a OpaqueBusMessage>,

    /// The outputs field should be empty, but the write 
    /// function will capture these outputs and add them
    /// to the bus
    outputs: HashMap<String, OpaqueBusMessage>,
}

impl<'a> StepContext<'a> {
    /// nanoseconds since start of the simmulation
    pub fn nanos(&self) -> u64 { 
        (self.sim_time - SIM_TIME_BEGIN).num_nanoseconds().unwrap() as u64
    }

    /// time in nanoseconds since last tick
    pub fn dt_nanos(&self) -> u64 { 
        self.sim_dt.num_nanoseconds().unwrap() as u64
    }

    /// time in seconds since last tick
    pub fn dt_secs(&self) -> f64 { 
        self.sim_dt.as_seconds_f64()
    }

    // Read from this step context to be ingested by your component. 
    // This function will allow you to read from an input port even 
    // if it is not connected to anything -- in such a case, 
    // it will return None.
    pub fn read<T: Signal>(&self, port: &InputPort<T>) -> Option<&T> {
        match self.inputs.get(port.get_id()) {
            Some(msg) => {
                match msg.inner.downcast_ref::<T>() {
                    Some(val) => Some(val),
                    None => {
                        tracing::warn!(
                            port_id = %port.get_id(),
                            expected_spec = %T::message_spec(),
                            actual_spec = %msg.message_spec,
                            "Downcast failed on StepContxt read - type mismatch"
                        );
                        None
                    }
                }
            },
            None => None 
        }
    }
    /// Write to this step context to be ingested by sim. This function will 
    /// allow you to write to an output port even if it is not connected 
    /// to anything.
    pub fn write<T: Signal>(&mut self, port: &OutputPort<T>, value: T) {
        let wrapped = BusMessage { 
            wall_time: Utc::now(),
            sim_time: self.sim_time,
            inner: value 
        };
        let boxed: OpaqueBusMessage = wrapped.into();
        let _ = self.outputs.insert(port.get_id().to_string(), boxed);
    }
}


/// Monolithic controller of simulation. Owns internal + external message
/// passing, as well as notion of global time. Calls update loops 
/// on behalf of components and propagates state updates where necessary.
/// 
/// Message passing currently works by taking a global snapshot 
/// of the message bus and passing along states to objects 
/// on next tick. 
///
/// TODO: use topological sort to reduce looping delays.
pub struct SimContext {
    global_nanos: u64,
    bus: Bus,
    components: Vec<Box<dyn Component>>,
}

impl SimContext {
    pub fn new() -> Self {
        Self {
            global_nanos: 0,
            bus: Bus::new(), 
            components: Vec::new(),            
        }
    }

    pub fn register_component(&mut self, component: Box<dyn Component>) -> Result<()> {
        self.components.push(component);
        Ok(())
    }

}
