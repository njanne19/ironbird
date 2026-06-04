use crate::message::{MessageSpec, Signal};
use crate::component::ComponentId;

use std::string::ToString;
use std::marker::PhantomData; 

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PortDirection {
    Input, 
    Output,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PortSpec {
    pub name: String, 
    pub direction: PortDirection,
    pub message_spec: MessageSpec,
}

pub trait Port {
    fn get_name(&self) -> &str;
    fn parent(&self) -> ComponentId;
    fn spec(&self) -> PortSpec;
}

/// Input port declaration struct. 
/// Only contains ID, but T indicates 
/// what data type this port expects.
pub struct InputPort<T: Signal> {
    parent: ComponentId,
    name: String,
    _input_type: PhantomData<T>,
}

impl<T: Signal> InputPort<T> {
    pub fn new(parent: ComponentId, name: &str) -> Self {
        Self {
            parent,
            name: name.to_string(),
            _input_type: PhantomData,
        }
    }
}

impl<T:Signal> Port for InputPort<T> {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn parent(&self) -> ComponentId {
        self.parent
    }

    fn spec(&self) -> PortSpec {
        PortSpec {
            name: self.name.clone(),
            direction: PortDirection::Input,
            message_spec: T::message_spec(),
        }
    }
}

/// Output port declaration struct. 
/// Only contains ID, but T indicates 
/// what data type this port expects to produce.
pub struct OutputPort<T: Signal> {
    parent: ComponentId,
    name: String,
    _output_type: PhantomData<T>,
}

impl<T: Signal> OutputPort<T> {
    pub fn new(parent: ComponentId, name: &str) -> Self {
        Self {
            parent,
            name: name.to_string(),
            _output_type: PhantomData,
        }
    }
}

impl<T: Signal> Port for OutputPort<T> {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn parent(&self) -> ComponentId {
        self.parent
    }

    fn spec(&self) -> PortSpec {
        PortSpec {
            name: self.name.clone(),
            direction: PortDirection::Output,
            message_spec: T::message_spec(),
        }
    }
}
