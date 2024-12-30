use serde_derive::{Deserialize, Serialize};
use zenoh_ros_derive::ZBytesCdr;

use crate::{builtin_interfaces::Time, std_msgs::Header};

pub mod control_mode_report {
    pub const NO_COMMAND: u8 = 0;
    pub const AUTONOMOUS: u8 = 1;
    pub const AUTONOMOUS_STEER_ONLY: u8 = 2;
    pub const AUTONOMOUS_VELOCITY_ONLY: u8 = 3;
    pub const MANUAL: u8 = 4;
    pub const DISENGAGED: u8 = 5;
    pub const NOT_READY: u8 = 6;
}
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct ControlModeReport {
    pub stamp: Time,
    pub mode: u8,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct Engage {
    pub stamp: Time,
    pub enable: bool,
}

pub mod gear_command {
    pub const NONE: u8 = 0;
    pub const NEUTRAL: u8 = 1;
    pub const DRIVE: u8 = 2;
    pub const DRIVE_2: u8 = 3;
    pub const DRIVE_3: u8 = 4;
    pub const DRIVE_4: u8 = 5;
    pub const DRIVE_5: u8 = 6;
    pub const DRIVE_6: u8 = 7;
    pub const DRIVE_7: u8 = 8;
    pub const DRIVE_8: u8 = 9;
    pub const DRIVE_9: u8 = 10;
    pub const DRIVE_10: u8 = 11;
    pub const DRIVE_11: u8 = 12;
    pub const DRIVE_12: u8 = 13;
    pub const DRIVE_13: u8 = 14;
    pub const DRIVE_14: u8 = 15;
    pub const DRIVE_15: u8 = 16;
    pub const DRIVE_16: u8 = 17;
    pub const DRIVE_17: u8 = 18;
    pub const DRIVE_18: u8 = 19;
    pub const REVERSE: u8 = 20;
    pub const REVERSE_2: u8 = 21;
    pub const PARK: u8 = 22;
    pub const LOW: u8 = 23;
    pub const LOW_2: u8 = 24;
}
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct GearCommand {
    pub stamp: Time,
    pub command: u8,
}

pub mod gear_report {
    pub const NONE: u8 = 0;
    pub const NEUTRAL: u8 = 1;
    pub const DRIVE: u8 = 2;
    pub const DRIVE_2: u8 = 3;
    pub const DRIVE_3: u8 = 4;
    pub const DRIVE_4: u8 = 5;
    pub const DRIVE_5: u8 = 6;
    pub const DRIVE_6: u8 = 7;
    pub const DRIVE_7: u8 = 8;
    pub const DRIVE_8: u8 = 9;
    pub const DRIVE_9: u8 = 10;
    pub const DRIVE_10: u8 = 11;
    pub const DRIVE_11: u8 = 12;
    pub const DRIVE_12: u8 = 13;
    pub const DRIVE_13: u8 = 14;
    pub const DRIVE_14: u8 = 15;
    pub const DRIVE_15: u8 = 16;
    pub const DRIVE_16: u8 = 17;
    pub const DRIVE_17: u8 = 18;
    pub const DRIVE_18: u8 = 19;
    pub const REVERSE: u8 = 20;
    pub const REVERSE_2: u8 = 21;
    pub const PARK: u8 = 22;
    pub const LOW: u8 = 23;
    pub const LOW_2: u8 = 24;
}
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct GearReport {
    pub stamp: Time,
    pub report: u8,
}

pub mod hazard_lights_command {
    pub const NO_COMMAND: u8 = 0;
    pub const DISABLE: u8 = 1;
    pub const ENABLE: u8 = 2;
}
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct HazardLightsCommand {
    pub stamp: Time,
    pub command: u8,
}

pub mod hazard_lights_report {
    pub const DISABLE: u8 = 1;
    pub const ENABLE: u8 = 2;
}
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct HazardLightsReport {
    pub stamp: Time,
    pub report: u8,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct SteeringReport {
    pub stamp: Time,
    pub steering_tire_angle: f32,
}

pub mod turn_indicators_command {
    pub const NO_COMMAND: u8 = 0;
    pub const DISABLE: u8 = 1;
    pub const ENABLE_LEFT: u8 = 2;
    pub const ENABLE_RIGHT: u8 = 3;
}
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct TurnIndicatorsCommand {
    pub stamp: Time,
    pub command: u8,
}

pub mod turn_indicators_report {
    pub const DISABLE: u8 = 1;
    pub const ENABLE_LEFT: u8 = 2;
    pub const ENABLE_RIGHT: u8 = 3;
}
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct TurnIndicatorsReport {
    pub stamp: Time,
    pub report: u8,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct VelocityReport {
    pub header: Header,
    pub longitudinal_velocity: f32,
    pub lateral_velocity: f32,
    pub heading_rate: f32,
}
