use crate::message::BusMessage;
use crate::{message::OpaqueBusMessage};
use crate::port::{InputPort, OutputPort};
use std::collections::{HashMap, HashSet};
use std::any::type_name;
use anyhow::{anyhow, Result};

/// A small context object that individual components
/// use to read/write inputs/outputs that they 
/// had previously defined in their component definition 
/// ONLY. Fields are private, accessible with the right
/// getters
pub struct StepContext<'a> {
    nanos: u64,
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
    // Read from this step context to be ingested by your component. 
    // This function will allow you to read from an input port even 
    // if it is not connected to anything -- in such a case, 
    // it will return None.
    pub fn read<T: Send + 'static>(&self, port: &InputPort<T>) -> Option<&T> {
        match self.inputs.get(port.get_id()) {
            Some(msg) => {
                match msg.inner.downcast_ref::<T>() {
                    Some(val) => Some(val),
                    None => {
                        tracing::warn!(
                            port_id = %port.get_id(),
                            expected_type = %type_name::<T>(),
                            actual_type = %msg.type_name,
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
    pub fn write<T: Send + 'static>(&mut self, port: &OutputPort<T>, value: T) {
        let wrapped = BusMessage { nanos: self.nanos, inner: value };
        let boxed: OpaqueBusMessage = wrapped.into();
        let _ = self.outputs.insert(port.get_id().to_string(), boxed);
    }
}
