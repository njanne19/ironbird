use crate::message::{OpaqueBusMessage, Signal};
use crate::channel::BusMessageChannel;

use std::collections::HashMap;
use anyhow::{anyhow, Result};

/// In-memory message bus for managing multiple channels. 
/// For writes: captures the value and stores it in its 
/// channels map (when the channel is alive). 
/// For reads: returns an immutable reference to the 
/// current (or previous) value currently in the 
/// channel map.
#[derive(Debug, Default)]
pub struct Bus {
   channels: HashMap<String, BusMessageChannel>, 
}

impl Bus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_new_channel<T: Signal>(&mut self, channel_id: String, ) -> Result<()> {
        if self.channels.contains_key(&channel_id) {
            return Err(anyhow!("Cannot add channel w/ID {} onto the bus \
                    because a channel with that ID already exists!", channel_id));
        }
        let new_channel = BusMessageChannel::new::<T>(channel_id.clone());
        self.channels.insert(channel_id, new_channel);
        Ok(())
    }

    pub fn write_latest_to_channel(&mut self, channel_id: &str, data: OpaqueBusMessage) -> Result<()> {
        match self.channels.get_mut(channel_id) {
            Some(channel) => {
                let _write_ok = channel.write(data)?;
                Ok(())
            },
            None => {
                Err(anyhow!("Cannot write to channel w/ID {} because it does not exist!",
                    channel_id))
            }
        }
    }

    pub fn read_current_from_channel(&self, channel_id: &str) -> Result<Option<&OpaqueBusMessage>> {
        match self.channels.get(channel_id) {
            Some(channel) => {
                Ok(channel.read())
            },
            None => {
                Err(anyhow!("Cannot read from channel w/ID {} because it does not exist!",
                    channel_id))
            }
        }
    }

    pub fn read_prev_from_channel(&self, channel_id: &str) -> Result<Option<&OpaqueBusMessage>> {
        match self.channels.get(channel_id) {
            Some(channel) => {
                Ok(channel.read_prev())
            },
            None => {
                Err(anyhow!("Cannot read from channel w/ID {} because it does not exist!",
                    channel_id))
            }
        }
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
    fn add_channel_write_and_read() {
        let mut bus = Bus::default();
        let now = Utc::now();
        let start_of_sim = NaiveTime::from_hms_opt(0, 0,0).unwrap();
        bus.add_new_channel::<u32>("ch".to_string()).unwrap();
        bus.write_latest_to_channel("ch", make_opaque(now, start_of_sim, 42u32)).unwrap();
        let msg = bus.read_current_from_channel("ch").unwrap().unwrap();
        assert_eq!(msg.wall_time, now);
    }

    #[test]
    fn add_duplicate_channel_errors() {
        let mut bus = Bus::default();
        bus.add_new_channel::<u32>("dup".to_string()).unwrap();
        assert!(bus.add_new_channel::<u32>("dup".to_string()).is_err());
    }

    #[test]
    fn write_to_nonexistent_channel_errors() {
        let mut bus = Bus::default();
        let now = Utc::now();
        let start_of_sim = NaiveTime::from_hms_opt(0, 0,0).unwrap();
        assert!(bus.write_latest_to_channel("nope", make_opaque(now, start_of_sim, 0u32)).is_err());
    }

    #[test]
    fn read_from_nonexistent_channel_errors() {
        let bus = Bus::default();
        assert!(bus.read_current_from_channel("nope").is_err());
        assert!(bus.read_prev_from_channel("nope").is_err());
    }

    #[test]
    fn read_empty_channel_returns_none() {
        let mut bus = Bus::default();
        bus.add_new_channel::<u32>("empty".to_string()).unwrap();
        assert!(bus.read_current_from_channel("empty").unwrap().is_none());
        assert!(bus.read_prev_from_channel("empty").unwrap().is_none());
    }

    #[test]
    fn read_prev_returns_previous_write() {
        let mut bus = Bus::default();
        bus.add_new_channel::<u32>("ch".to_string()).unwrap();
        let now = Utc::now();
        let start_of_sim = NaiveTime::from_hms_opt(0, 0,0).unwrap();
        bus.write_latest_to_channel("ch", make_opaque(now, start_of_sim, 1u32)).unwrap();
        bus.write_latest_to_channel("ch", make_opaque(now + TimeDelta::nanoseconds(3), start_of_sim + TimeDelta::seconds(3), 2u32)).unwrap();
        let prev = bus.read_prev_from_channel("ch").unwrap().unwrap();
        assert_eq!(prev.wall_time, now);
    }
}
