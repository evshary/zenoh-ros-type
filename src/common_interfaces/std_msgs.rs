use serde_derive::{Deserialize, Serialize};

use crate::builtin_interfaces::Time;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Header {
    pub stamp: Time,
    pub frame_id: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct ColorRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
