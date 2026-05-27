use crate::message::{OpaqueBusMessage, MessageSpec, Signal};
use anyhow::{anyhow, Result};

/// A type-erased BusMessageChannel used for orchestration. Channels 
/// are where pub/sub data is stored. While BusMessageChannel itself does 
/// not provide different implementations over different types T, it 
/// captures a type id and name to a specific type T. Meaning each 
/// construction of BusMessageChannel supports only a single type at runtime.
#[derive(Debug)]
pub struct BusMessageChannel {
    id: String,
    current: Option<OpaqueBusMessage>,
    prev: Option<OpaqueBusMessage>,
    num_writes: u32,
    message_spec: MessageSpec,
}

impl BusMessageChannel {
    /// Typed constructor for BusMessageChannel.
    /// specifying the T parameter here defines
    /// which message data types are valid on this 
    /// channel.
    pub fn new<T: Signal>(id: String) -> Self {
        Self {
            id,
            current: None,
            prev: None, 
            num_writes: 0,
            message_spec: T::message_spec(),
        }
    }

    /// Write function for BusMessageChannel. Takes ownership 
    /// of the passed in write, and stores it in its internal state.
    pub fn write(&mut self, data: OpaqueBusMessage) -> Result<()> {
        if data.message_spec != self.message_spec {
            return Err(anyhow!("Attempted to publish data of spec {:#?} to channel \
            w/spec {:#?}.", data.message_spec, self.message_spec))
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
    use chrono::{DateTime, Utc, NaiveTime, TimeDelta};

    fn make_opaque<T: Signal>(wall_time: DateTime<Utc>, sim_time: chrono::NaiveTime, inner: T) -> OpaqueBusMessage {
        BusMessage { wall_time, sim_time, inner }.into()
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
        let now = Utc::now();
        let start_of_sim = NaiveTime::from_hms_opt(0, 0,0).unwrap();
        ch.write(make_opaque(now, start_of_sim, 42u32)).unwrap();
        assert_eq!(ch.read().unwrap().wall_time, now);
    }

    #[test]
    fn second_write_advances_prev() {
        let mut ch = BusMessageChannel::new::<u32>("test".to_string());
        let now = Utc::now();
        let start_of_sim = NaiveTime::from_hms_opt(0, 0,0).unwrap();
        ch.write(make_opaque(now, start_of_sim, 1u32)).unwrap();
        ch.write(make_opaque(
                now + TimeDelta::nanoseconds(3),
                start_of_sim + TimeDelta::seconds(3),
                2u32)
        ).unwrap();
        assert_eq!(ch.read().unwrap().wall_time, now + TimeDelta::nanoseconds(3));
        assert_eq!(ch.read_prev().unwrap().sim_time, start_of_sim);
    }

    #[test]
    fn write_wrong_type_returns_error() {
        let mut ch = BusMessageChannel::new::<u32>("test".to_string());
        let now = Utc::now();
        let start_of_sim = NaiveTime::from_hms_opt(0, 0,0).unwrap();
        assert!(ch.write(make_opaque(now, start_of_sim, 1.0f64)).is_err());
    }

    #[test]
    fn prev_is_none_after_single_write() {
        let mut ch = BusMessageChannel::new::<u32>("test".to_string());
        let now = Utc::now();
        let start_of_sim = NaiveTime::from_hms_opt(0, 0,0).unwrap();
        ch.write(make_opaque(now, start_of_sim, 42u32)).unwrap();
        assert!(ch.read_prev().is_none());
    }
}
