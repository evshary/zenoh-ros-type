use serde_derive::{Deserialize, Serialize};

use crate::builtin_interfaces::Time;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Control {
    pub stamp: Time,
    pub control_time: Time,
    pub lateral: Lateral,
    pub longitudinal: Longitudinal,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Lateral {
    pub stamp: Time,
    pub control_time: Time,
    pub steering_tire_angle: f32,
    pub steering_tire_rotation_rate: f32,
    pub is_defined_steering_tire_rotation_rate: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct ControlHorizon {
    pub stamp: Time,
    pub control_time: Time,
    pub time_step_ms: f32,
    pub controls: Control,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Longitudinal {
    pub stamp: Time,
    pub control_time: Time,
    pub velocity: f32,
    pub acceleration: f32,
    pub jerk: f32,
    pub is_defined_acceleration: bool,
    pub is_defined_jerk: bool,
}
