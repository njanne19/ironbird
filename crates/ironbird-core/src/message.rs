/// Definition of ironbird signals -- messages that are 
/// able to carry information to and from the 
/// internal message bus.
use std::any::{Any, TypeId, type_name};
use chrono::{DateTime, NaiveTime, Utc};
use anyhow::{anyhow, Result};
use std::fmt;

/// A struct that contains both 
/// an internally referencable 
/// and an externally (stable)
/// referencable type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageSpec {
    pub internal_type_id: TypeId,
    pub internal_type_name: &'static str,
    pub stable_type_name: &'static str,
}

impl fmt::Display for MessageSpec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Message spec: (internal_type_id: {:#?}, internal_type_name: {}, stable_type_name: {})", 
            self.internal_type_id, self.internal_type_name, self.stable_type_name)
    }
}

/// A data type is OK for the bus if it 
/// implements Signal. That is, 
/// it can be sent across threads, castable down 
/// to Box<dyn Any>, and emits a message spec
pub trait Signal: Send + 'static {
    const STABLE_TYPE_NAME: &'static str;
    fn message_spec() -> MessageSpec {
        MessageSpec {
            internal_type_id: TypeId::of::<Self>(),
            internal_type_name: type_name::<Self>(),
            stable_type_name: Self::STABLE_TYPE_NAME,
        }
    }
}

/// Some macro magic to cover some basic types and their trivial signal implementations
macro_rules! impl_signal_trivial {
    ($($t:ty)*) => {
       $(
           impl Signal for $t {
               const STABLE_TYPE_NAME: &'static str = concat!("ironbird/", stringify!($t));
           }
       )*
    };
}
impl_signal_trivial!(f64 f32 u64 u32 usize bool);

/// A typed bus message. Owns
/// internal data, and attaches 
/// a sim-based timestamp
#[derive(Debug, Clone, Copy)]
pub struct BusMessage<T: Signal> { 
    pub wall_time: DateTime<Utc>,
    pub sim_time: NaiveTime,
    pub inner: T,
}

/// A type-erased bus message used 
/// for orchestration.
#[derive(Debug)]
pub struct OpaqueBusMessage {
    pub wall_time: DateTime<Utc>,
    pub sim_time: NaiveTime,
    pub message_spec: MessageSpec,
    pub inner: Box<dyn Any>,
}

impl<T: Signal> From<BusMessage<T>> for OpaqueBusMessage {
    fn from(msg: BusMessage<T>) -> Self {
        Self {
            wall_time: msg.wall_time,
            sim_time: msg.sim_time,
            message_spec: T::message_spec(),
            inner: Box::new(msg.inner)
        }  
    }
}

impl<T: Signal> TryFrom<OpaqueBusMessage> for BusMessage<T> {
    type Error = anyhow::Error;
    fn try_from(msg: OpaqueBusMessage) -> Result<Self, Self::Error> {
        match msg.inner.downcast::<T>() { 
            Ok(dc) => Ok(Self {
                wall_time: msg.wall_time,
                sim_time: msg.sim_time,
                inner: *dc,
            }),
            Err(_not_dc) => {
                return Err(anyhow!("Failed to convert from OpaqueBusMessage w/spec {:#?} to BusMessage<{}>. Are you sure you called this function correctly?",
                    msg.message_spec, type_name::<T>()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opaque_from_bus_message_preserves_fields() {
        let now = Utc::now();
        let start_of_sim = NaiveTime::from_hms_opt(0, 0,0).unwrap();
        let msg: BusMessage<u32> = BusMessage { 
            wall_time: now, 
            sim_time: start_of_sim,
            inner: 99u32 
        };
        let opaque: OpaqueBusMessage = msg.into();
        assert_eq!(opaque.wall_time, now);
        assert_eq!(opaque.sim_time, start_of_sim);
        assert_eq!(opaque.message_spec, u32::message_spec())
    }

    #[test]
    fn bus_message_try_from_opaque_roundtrips() {
        let now = Utc::now();
        let start_of_sim = NaiveTime::from_hms_opt(0, 0,0).unwrap();
        let msg: BusMessage<u32> = BusMessage { 
            wall_time: now, 
            sim_time: start_of_sim,
            inner: 123u32
        };
        let opaque: OpaqueBusMessage = msg.into();
        let result: BusMessage<u32> = opaque.try_into().unwrap();
        assert_eq!(result.wall_time, now);
        assert_eq!(result.sim_time, start_of_sim);
        assert_eq!(result.inner, 123u32);
    }

    #[test]
    fn bus_message_try_from_opaque_wrong_type_errors() {
        let now = Utc::now();
        let start_of_sim = NaiveTime::from_hms_opt(0, 0,0).unwrap();
        let msg: BusMessage<u32> = BusMessage {
            wall_time: now, 
            sim_time: start_of_sim,
            inner: 1u32
        };
        let opaque: OpaqueBusMessage = msg.into();
        let result: Result<BusMessage<f64>, _> = opaque.try_into();
        assert!(result.is_err());
    }
}
