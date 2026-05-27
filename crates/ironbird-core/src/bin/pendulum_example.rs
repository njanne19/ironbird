use ironbird_core::{
    component::examples::TorquedPendulumPhysics2D,
    component::examples::PD2To1Controller,
    sim_context:: SimContext,
    types::StateVec,
    types::World,
};

pub fn main() {

    let sim = SimContext::new();

    let pendulum = TorquedPendulumPhysics2D::new(
        1.0,
        1.0,
        StateVec::<f64, 2, World>::new([0.0, 0.0])
    );

    let controller = PD2To1Controller::new(
        1.0, 
        0.1, 
        std::f64::consts::PI, 
        0.0,
    );


}
