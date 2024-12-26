/// The struct is used by ROS action.
/// If you want to sent ROS action with Zenoh directly. You should include the header.
/// Refer to https://design.ros2.org/articles/actions.html for more detail.
use serde_derive::{Deserialize, Serialize};

use crate::builtin_interfaces::Time;

/// The response struct for Action SendGoal
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct ActionSendGoalResponse {
    pub accept: bool, // Accept the request or not
    pub timestamp: Time,
}

/// The request struct for getting ActionResult
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct ActionResultRequest {
    pub goal_id: [u8; 16],
}

/// The request struct for cancelling goal service
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct ActionCancelRequest {
    pub goal_id: [u8; 16],
    pub timestamp: Time,
}

/// The response struct for cancelling goal service
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct ActionCancelResponse {
    pub response_code: i8, // OK, REJECTED or INVALID_GOAL_ID
    pub goal_id: Vec<[u8; 16]>,
}

pub mod action_status {
    pub const UNKNOWN: i8 = 0;
    pub const ACCEPTED: i8 = 1;
    pub const EXECUTING: i8 = 2;
    pub const CANCELING: i8 = 3;
    pub const SUCCEEDED: i8 = 4;
    pub const CANCELED: i8 = 5;
    pub const ABORTED: i8 = 6;
}

/// Goal Status Topic
#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct ActionStatus {
    pub goal_id: Vec<[u8; 16]>,
    pub timestamp: Time,
    // Refer to action_status
    pub status: i8,
}
