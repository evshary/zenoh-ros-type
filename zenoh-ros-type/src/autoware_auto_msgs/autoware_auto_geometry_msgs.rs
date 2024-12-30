use serde_derive::{Deserialize, Serialize};
use zenoh_ros_derive::ZBytesCdr;

use crate::{geometry_msgs::Point, std_msgs::Header};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct Complex32 {
    pub real: f32,
    pub imag: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct Quaternion32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct RelativePositionWithCovarianceStamped {
    pub header: Header,
    pub child_frame_id: String,
    pub position: Point,
    pub covariance: [f64; 9],
}
