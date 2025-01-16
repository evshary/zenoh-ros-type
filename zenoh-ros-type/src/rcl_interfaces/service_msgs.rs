use serde_derive::{Deserialize, Serialize};
use zenoh_ros_derive::ZBytesCdr;

use crate::builtin_interfaces::Time;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct ServiceEventInfo {
    pub event_type: u8,
    pub stamp: Time,
    pub client_gid: [char; 16],
    pub sequence_number: i64,
}
