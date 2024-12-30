use serde_derive::{Deserialize, Serialize};
use zenoh_ros_derive::ZBytesCdr;

use crate::builtin_interfaces::Time;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Header {
    pub stamp: Time,
    pub frame_id: String,
}

// To avoid conflict with the basic type in Rust, we add underscore
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct String_ {
    pub data: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ColorRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
