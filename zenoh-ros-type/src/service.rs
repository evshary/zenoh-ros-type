/// The struct is used by ROS service.
/// If you want to sent ROS service with Zenoh directly. You should include the header.
use serde_derive::{Deserialize, Serialize};
use zenoh_ros_derive::ZBytesCdr;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct ServiceHeader {
    pub guid: i64,
    pub seq: u64,
}
