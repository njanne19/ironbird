use std::any::{Any, TypeId, type_name};
use anyhow::{anyhow, Result};


/// A typed bus message. Must contain 
/// strictly owned data.
pub struct BusMessage<T: Send + 'static> { 
    nanos: u64,
    inner: T,
}

/// A type-erased bus message used 
/// for orchestration.
pub struct OpaqueBusMessage {
    nanos: u64,
    type_id: TypeId,
    type_name: &'static str,
    inner: Box<dyn Any + Send>,
}

impl<T: Send + 'static> From<BusMessage<T>> for OpaqueBusMessage {
    fn from(msg: BusMessage<T>) -> Self {
        Self {
            nanos: msg.nanos,
            type_id: TypeId::of::<T>(),
            type_name: type_name::<T>(),
            inner: Box::new(msg.inner)
        }  
    }
}

impl<T: Send + 'static> TryFrom<OpaqueBusMessage> for BusMessage<T> {
    type Error = anyhow::Error;
    fn try_from(msg: OpaqueBusMessage) -> Result<Self, Self::Error> {
        match msg.inner.downcast::<T>() { 
            Ok(dc) => Ok(Self {
                nanos: msg.nanos,
                inner: *dc,
            }),
            Err(_not_dc) => {
                return Err(anyhow!("Failed to convert from OpaqueBusMessage w/type {} to BusMessage<{}>. Are you sure you called this function correctly?",
                    msg.type_name, type_name::<T>()))
            }
        }
    }
}
