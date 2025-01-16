use serde_derive::{Deserialize, Serialize};
use zenoh_ros_derive::ZBytesCdr;

use crate::{builtin_interfaces::Time, unique_identifier_msgs::UUID};

// ----------------------- msg -----------------------

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct GoalInfo {
    pub goal_id: UUID,
    pub stamp: Time,
}

pub mod goal_status {
    pub const UNKNOWN: i8 = 0;
    pub const ACCEPTED: i8 = 1;
    pub const EXECUTING: i8 = 2;
    pub const CANCELING: i8 = 3;
    pub const SUCCEEDED: i8 = 4;
    pub const CANCELED: i8 = 5;
    pub const ABORTED: i8 = 6;
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct GoalStatus {
    pub goal_info: GoalInfo,
    pub status: i8,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct GoalStatusArray {
    pub status_list: Vec<GoalStatus>,
}

// ----------------------- srv -----------------------

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct CancelGoalRequest {
    pub goal_info: GoalInfo,
}

pub mod cancel_goal_return_code {
    pub const NONE: i8 = 0;
    pub const REJECTED: i8 = 1;
    pub const UNKNOWN_GOAL_ID: i8 = 2;
    pub const GOAL_TERMINATED: i8 = 3;
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct CancelGoalResponse {
    pub return_code: i8,
    pub goals_canceling: Vec<GoalInfo>,
}
