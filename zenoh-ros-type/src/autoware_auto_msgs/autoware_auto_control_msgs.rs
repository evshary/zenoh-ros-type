use serde_derive::{Deserialize, Serialize};
use zenoh_ros_derive::ZBytesCdr;

use crate::builtin_interfaces::Time;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct AckermannControlCommand {
    pub stamp: Time,
    pub lateral: AckermannLateralCommand,
    pub longitudinal: LongitudinalCommand,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct AckermannLateralCommand {
    pub stamp: Time,
    pub steering_tire_angle: f32,
    pub steering_tire_rotation_rate: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct HighLevelControlCommand {
    pub stamp: Time,
    pub velocity_mps: f32,
    pub curvature: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct LongitudinalCommand {
    pub stamp: Time,
    pub speed: f32,
    pub acceleration: f32,
    pub jerk: f32,
}
