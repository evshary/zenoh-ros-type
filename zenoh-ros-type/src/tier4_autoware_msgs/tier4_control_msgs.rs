use serde_derive::{Deserialize, Serialize};
use zenoh_ros_derive::ZBytesCdr;

pub mod gate_mode_data {
    pub const AUTO: u8 = 0;
    pub const EXTERNAL: u8 = 1;
}
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct GateMode {
    pub data: u8,
}
