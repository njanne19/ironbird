use uuid::Uuid;
use std::string::ToString;
use std::marker::PhantomData; 

/// Input port declaration struct. 
/// Only contains ID, but T indicates 
/// what data type this port expects.
pub struct InputPort<T: Send + 'static> {
    id: String,
    _input_type: PhantomData<T>,
}

impl<T: Send + 'static> InputPort<T> {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            _input_type: PhantomData,
        }
    }

    pub fn with_name(name: &str) -> Self {
        Self {
            id: name.to_string(),
            _input_type: PhantomData,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}

/// Output port declaration struct. 
/// Only contains ID, but T indicates 
/// what data type this port expects to produce.
pub struct OutputPort<T: Send + 'static> {
    id: String,
    _output_type: PhantomData<T>,
}

impl<T: Send + 'static> OutputPort<T> {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            _output_type: PhantomData,
        }
    }

    pub fn with_name(name: &str) -> Self {
        Self {
            id: name.to_string(),
            _output_type: PhantomData,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}
