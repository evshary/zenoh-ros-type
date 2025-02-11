use serde_derive::{Deserialize, Serialize};
use zenoh_ros_derive::ZBytesCdr;

use crate::std_msgs::Header;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct ActuationCommand {
    pub accel_cmd: f64,
    pub brake_cmd: f64,
    pub steer_cmd: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct ActuationCommandStamped {
    pub header: Header,
    pub actuation: ActuationCommand,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct ActuationStatus {
    pub accel_status: f64,
    pub brake_status: f64,
    pub steer_status: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct ActuationStatusStamped {
    pub header: Header,
    pub status: ActuationStatus,
}