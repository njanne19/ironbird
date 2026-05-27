use crate::sim_context::StepContext;
use crate::port::{InputPort, OutputPort};
use crate::types::*;
use anyhow::{anyhow, Result};

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
pub trait Component {
    fn step(&mut self, ctx: &mut StepContext) -> Result<()>;
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
            Self {
                mass_kg,
                suspension_length_m,
                ang_state: initial_ang_state,
                pivot_torque_in: InputPort::new("pivot_torque_in"),
                ang_state_out: OutputPort::new("ang_state_out"),
                trans_state_out: OutputPort::new("trans_state_out"),
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
    }

    /// Simple PD 2DoF State to 1DoF output controller
    pub struct PD2To1Controller {
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
            Self {
                kp,
                kd, 
                theta_target,
                theta_target_dot,
                last_error: 0.0,
                last_error_dot: 0.0,
                ang_state_in: InputPort::new("ang_state_in"),
                torque_out: OutputPort::new("torque_out"),
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
    }

} 
