use crate::message::{OpaqueBusMessage};
use std::any::{TypeId, type_name};
use anyhow::{anyhow, Result};

/// An type-erased BusMessageChannel
/// used for orchestration. Channels 
/// are where pub/sub data is stored. 
/// While BusMessageChannel itself does 
/// not provide different implementations 
/// over different types T, it 
/// captures a type id and name to 
/// a specific type T. Meaning each 
/// construction of BusMessageChannel 
/// supports only a single type at runtime.
#[derive(Debug)]
pub struct BusMessageChannel {
    id: String,
    current: Option<OpaqueBusMessage>,
    prev: Option<OpaqueBusMessage>,
    num_writes: u32,
    type_id: TypeId,
    type_name: &'static str,
}

impl BusMessageChannel {
    /// Typed constructor for BusMessageChannel.
    /// specifying the T parameter here defines
    /// which message data types are valid on this 
    /// channel.
    pub fn new<T: Send + 'static>(id: String) -> Self {
        Self {
            id,
            current: None,
            prev: None, 
            num_writes: 0,
            type_id: TypeId::of::<T>(),
            type_name: type_name::<T>(),
        }
    }

    /// Write function for BusMessageChannel. Takes ownership 
    /// of the passed in write, and stores it in its internal state.
    pub fn write(&mut self, data: OpaqueBusMessage) -> Result<()> {
        if data.type_id != self.type_id {
            return Err(anyhow!("Attempted to publish data of type {} to channel \
            w/id {}. Channel has type {}.", data.type_name, self.id, self.type_name))
        }
        self.prev = self.current.take();
        self.current = Some(data);
        self.num_writes += 1;
        Ok(())
    }
    
    /// Read function for BusMessageChannel. Returns an immutable 
    /// reference to the current item in message state.
    pub fn read(&self) -> Option<&OpaqueBusMessage> {
        self.current.as_ref()
    }

    /// Read prev function for BusMessageChannel. Returns an immutable 
    /// reference to the message just before current (the value rendered
    /// if `read` was called).
    pub fn read_prev(&self) -> Option<&OpaqueBusMessage> {
        self.prev.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::BusMessage;

    fn make_opaque<T: Send + 'static>(nanos: u64, inner: T) -> OpaqueBusMessage {
        BusMessage { nanos, inner }.into()
    }

    #[test]
    fn new_channel_starts_empty() {
        let ch = BusMessageChannel::new::<u32>("test".to_string());
        assert!(ch.read().is_none());
        assert!(ch.read_prev().is_none());
    }

    #[test]
    fn write_and_read_current() {
        let mut ch = BusMessageChannel::new::<u32>("test".to_string());
        ch.write(make_opaque(1, 42u32)).unwrap();
        assert_eq!(ch.read().unwrap().nanos, 1);
    }

    #[test]
    fn second_write_advances_prev() {
        let mut ch = BusMessageChannel::new::<u32>("test".to_string());
        ch.write(make_opaque(10, 1u32)).unwrap();
        ch.write(make_opaque(20, 2u32)).unwrap();
        assert_eq!(ch.read().unwrap().nanos, 20);
        assert_eq!(ch.read_prev().unwrap().nanos, 10);
    }

    #[test]
    fn write_wrong_type_returns_error() {
        let mut ch = BusMessageChannel::new::<u32>("test".to_string());
        assert!(ch.write(make_opaque(0, 1.0f64)).is_err());
    }

    #[test]
    fn prev_is_none_after_single_write() {
        let mut ch = BusMessageChannel::new::<u32>("test".to_string());
        ch.write(make_opaque(1, 42u32)).unwrap();
        assert!(ch.read_prev().is_none());
    }
}
