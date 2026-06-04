use crate::sim_context::StepContext;
use crate::port::{InputPort, OutputPort, PortSpec, Port};
use crate::message::Signal;
use crate::types::*;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::any::{type_name, TypeId};

/// A component can be as simple as you want it to be. 
/// It can also be as complex as you want it to be.
/// You can use ports to describe inputs and outputs 
/// (accessible via StepContext), as well as maintain
/// internal state (which is provided by the mutable refernce
/// to self). 
///
/// Component inputs/outputs are wired together 
/// via sim context connect handle. Sim context 
/// will eventually call individual components' 
/// step functions in the appropriate order 
/// by dependency.
pub trait Component: 'static {
    fn step(&mut self, ctx: &mut StepContext) -> Result<()>;
    fn core(&self) -> &ComponentCore;
    fn core_mut(&mut self) -> &mut ComponentCore;
    fn id(&self) -> ComponentId { self.core().id() }
    fn spec(&self) -> ComponentSpec { self.core().spec() }

    // Has to be called before being added to the simulation context, 
    // which is probably what you want to begin with.
    fn connect<L: Port, R: Port>(&mut self, left: &L, right: &R) -> Result<()> 
    where Self: Sized
    {
        // If one of these ports is owned by the component, make an entry 
        // in its connection table.
        if left.parent() != self.id() && right.parent() != self.id() {
            return Err(anyhow!("Attempted to connect two ports that are neither tied to this component!"));
        }
        let self_id = self.id();
        let core = self.core_mut();
        if let Some(prev) = core.connection_table.insert(left.spec(), right.spec()) {
            tracing::warn!("New port connection made for component w/id {:#?} between {:#?} and {:#?}, overwriting previous.",
                self_id, left.spec(), right.spec());
            tracing::warn!("Was previously: {:#?} and {:#?}", left.spec(), prev);
        } else {
            tracing::info!("New port connection made for component w/id {:#?} between {:#?} and {:#?}",
                self_id, left.spec(), right.spec());
        };

        Ok(())
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComponentId {
    instance: u64,
    type_id: TypeId,
    type_name: &'static str,
}

impl ComponentId {
    pub fn new<T: 'static>() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        ComponentId {
            instance: NEXT.fetch_add(1, Ordering::Relaxed),
            type_id: TypeId::of::<T>(),
            type_name: type_name::<T>(),
        }
    }
}

pub struct ComponentCore {
    id: ComponentId,
    inputs: Vec<PortSpec>,
    outputs: Vec<PortSpec>,
    connection_table: HashMap<PortSpec, PortSpec>,
}

impl ComponentCore {
    pub fn new<CompType: 'static>() -> Self {
        ComponentCore { 
            id: ComponentId::new::<CompType>(), 
            inputs: vec![], 
            outputs: vec![],
            connection_table: HashMap::new(),
        }
    }

    pub fn add_input<T: Signal>(&mut self, name: &str) -> InputPort<T> {
        let port = InputPort::new(self.id, name);
        self.inputs.push(port.spec());
        port
    }

    pub fn add_output<T: Signal>(&mut self, name: &str) -> OutputPort<T> {
        let port = OutputPort::new(self.id, name);
        self.outputs.push(port.spec());
        port
    }

    pub fn id(&self) -> ComponentId { self.id }
    pub fn spec(&self) -> ComponentSpec {
        ComponentSpec {
            component_type: self.id.type_name.to_string(),
            inputs: self.inputs.clone(),
            outputs: self.outputs.clone(),
        }
    }
}

/// A serializable summary of a component, 
/// which is a combination of its type, 
/// its inputs, and its outputs.
/// TODO: add more here later if necessary.
#[derive(Debug, Clone)]
pub struct ComponentSpec {
    pub component_type: String,
    pub inputs: Vec<PortSpec>, 
    pub outputs: Vec<PortSpec>,
}

pub mod examples {
    use super::*;

    /// Simple pendulum experiment. 
    ///
    /// Assumptions:
    /// - Angular state is initialized to 0, 
    /// which is suspension_length away 
    /// in negative Y below the pivot point (origin)
    /// - Suspension bar is rigid and massless
    /// Note that angular state 
    /// - Torque can be applied at the pivot joint.
    pub struct TorquedPendulumPhysics2D {
        core: ComponentCore,

        // physical properties
        mass_kg: f64,
        suspension_length_m: f64,

        // TODO: figure out a better way to address units here
        // theta + theta dot (radians)
        ang_state: StateVec<f64, 2, World>,

        // Input Channel definitions
        pub pivot_torque_in: InputPort<TorqueNm>,
        
        // Output Channel definitions
        pub ang_state_out: OutputPort<StateVec<f64, 2, World>>,
        pub trans_state_out: OutputPort<StateVec<f64, 4, World>>,
    }

    // geom helper
    impl TorquedPendulumPhysics2D {
        fn to_trans_state(&self) -> StateVec<f64, 4, World> {
            StateVec::<f64, 4, World>::new([
                    self.suspension_length_m * self.ang_state.data[0].sin(),
                    -1.0 * self.suspension_length_m * self.ang_state.data[0].cos(),
                    self.suspension_length_m * self.ang_state.data[1].cos(),
                    self.suspension_length_m * self.ang_state.data[1].sin(),
            ])
        }

        pub fn new(
            mass_kg: f64,
            suspension_length_m: f64,
            initial_ang_state: StateVec<f64, 2, World>,
        ) -> Self {
            let mut core = ComponentCore::new::<Self>();
            Self {
                pivot_torque_in: core.add_input("torque"),
                ang_state_out: core.add_output("angular_state"),
                trans_state_out: core.add_output("cartesian_state"),
                core,
                mass_kg,
                suspension_length_m,
                ang_state: initial_ang_state,
            }
        }
    }

    // Uses euler integration here. May not be the 
    // best long term.
    impl Component for TorquedPendulumPhysics2D {
        fn step(&mut self, ctx: &mut StepContext) -> Result<()> {

            // Get the input to update the physics.
            let dt = ctx.dt_secs();
            let torque = ctx.read(&self.pivot_torque_in)
                .ok_or(anyhow!("Failed to read input torque value from step context"))?
                .0;

            // update theta based on prev velocity.
            let new_theta = self.ang_state.data[0] + dt * self.ang_state.data[1];

            // update new velocity based on incoming torque
            // TODO: add grav constants
            let new_theta_dot = self.ang_state.data[1] + 
                dt * ((-1.0 * 9.81 / self.suspension_length_m) * self.ang_state.data[0].sin() + torque/(self.mass_kg * (self.suspension_length_m.powi(2))));

            self.ang_state.data = [new_theta, new_theta_dot];

            ctx.write(&self.ang_state_out, self.ang_state.clone());
            ctx.write(&self.trans_state_out, self.to_trans_state());

            Ok(())
        }
        fn core(&self) -> &ComponentCore { &self.core }
        fn core_mut(&mut self) -> &mut ComponentCore { &mut self.core }
    }

    /// Simple PD 2DoF State to 1DoF output controller
    pub struct PD2To1Controller {
        core: ComponentCore,

        // physical properties
        kp: f64, 
        kd: f64,

        theta_target: f64,
        theta_target_dot: f64,

        last_error: f64,
        last_error_dot: f64,

        // Input Channel definitions
        ang_state_in: InputPort<StateVec<f64, 2, World>>,
        
        // Output Channel definitions
        torque_out: OutputPort<TorqueNm>,
    }

    impl PD2To1Controller {
        pub fn new(
            kp: f64,
            kd: f64,
            theta_target: f64,
            theta_target_dot: f64,
        ) ->Self {
            let mut core = ComponentCore::new::<Self>();
            Self {
                ang_state_in: core.add_input("angular_state"),
                torque_out: core.add_output("torque"),
                core,
                kp,
                kd, 
                theta_target,
                theta_target_dot,
                last_error: 0.0,
                last_error_dot: 0.0,
            }
        }
    }

    impl Component for PD2To1Controller {
        fn step(&mut self, ctx: &mut StepContext) -> Result<()> {
            
            // Get the input state from the pendulum.
            let ang_state = ctx.read(&self.ang_state_in)
                .ok_or(anyhow!("Failed to read input angular value from step context"))?
                .data;

            let current_error = self.theta_target - ang_state[0];
            let current_error_dot = self.theta_target_dot - ang_state[1];

            // update errors for next cycle.
            self.last_error = current_error;
            self.last_error_dot = current_error_dot;

            ctx.write(&self.torque_out, TorqueNm(
                self.kp * current_error + self.kd * current_error_dot
            ));

            Ok(())
        }
        fn core(&self) -> &ComponentCore { &self.core }
        fn core_mut(&mut self) -> &mut ComponentCore { &mut self.core }
    }

} 
