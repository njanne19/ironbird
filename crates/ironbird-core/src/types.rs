use crate::message::Signal;

/// Frame definitions
#[derive(Debug, Clone, Copy)]
pub struct World;

/// Vector of N states of type T, locked 
/// to a frame called Frame.
#[derive(Debug, Clone, Copy)]
pub struct StateVec<T, const N: usize, Frame> {
    pub data: [T; N],
    pub _frame: std::marker::PhantomData<Frame>,
}

impl<T, const N: usize, Frame> StateVec<T, N, Frame> {
    pub fn new(data: [T; N]) -> Self {
        Self {
            data,
            _frame: std::marker::PhantomData::<Frame>,
        }
    }
}
impl Signal for StateVec<f64, 2, World> {
    const STABLE_TYPE_NAME: &'static str = "irongbird/StateVec<f64,2,World>";
}
impl Signal for StateVec<f64, 4, World> {
    const STABLE_TYPE_NAME: &'static str = "irongbird/StateVec<f64,4,World>";
}


pub struct TorqueNm(pub f64);
impl Signal for TorqueNm {
    const STABLE_TYPE_NAME: &'static str = "ironbird/TorqueNm";
}
