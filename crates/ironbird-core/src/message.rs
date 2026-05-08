use std::any::{Any, TypeId, type_name};
use anyhow::{anyhow, Result};

/// A typed bus message. Must contain 
/// strictly owned data.
#[derive(Debug, Clone, Copy)]
pub struct BusMessage<T: Send + 'static> { 
    pub nanos: u64,
    pub inner: T,
}

/// A type-erased bus message used 
/// for orchestration.
#[derive(Debug)]
pub struct OpaqueBusMessage {
    pub nanos: u64,
    pub type_id: TypeId,
    pub type_name: &'static str,
    pub inner: Box<dyn Any + Send>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opaque_from_bus_message_preserves_fields() {
        let msg: BusMessage<u32> = BusMessage { nanos: 42, inner: 99u32 };
        let opaque: OpaqueBusMessage = msg.into();
        assert_eq!(opaque.nanos, 42);
        assert_eq!(opaque.type_id, TypeId::of::<u32>());
        assert_eq!(opaque.type_name, type_name::<u32>());
    }

    #[test]
    fn bus_message_try_from_opaque_roundtrips() {
        let msg: BusMessage<u32> = BusMessage { nanos: 7, inner: 123u32 };
        let opaque: OpaqueBusMessage = msg.into();
        let result: BusMessage<u32> = opaque.try_into().unwrap();
        assert_eq!(result.nanos, 7);
        assert_eq!(result.inner, 123u32);
    }

    #[test]
    fn bus_message_try_from_opaque_wrong_type_errors() {
        let msg: BusMessage<u32> = BusMessage { nanos: 0, inner: 1u32 };
        let opaque: OpaqueBusMessage = msg.into();
        let result: Result<BusMessage<f64>, _> = opaque.try_into();
        assert!(result.is_err());
    }
}
