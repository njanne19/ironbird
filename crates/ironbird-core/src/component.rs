use crate::sim_context::StepContext;

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
    fn step(&mut self, ctx: &mut StepContext);
}
