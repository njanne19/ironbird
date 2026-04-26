use serde::{Serialize, Deserialize};
use specta::Type;

// Example type
#[derive(Serialize, Deserialize, Type)]
pub struct SimState {
    pub position: [f32; 3],
    pub attitude: [f32; 4],
    pub velocity: [f32; 3],
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
