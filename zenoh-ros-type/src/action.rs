/// The struct is used by ROS action.
/// If you want to sent ROS action with Zenoh directly. You should include the header.
/// Refer to https://design.ros2.org/articles/actions.html for more detail.
use serde_derive::{Deserialize, Serialize};
use zenoh_ros_derive::ZBytesCdr;

use crate::builtin_interfaces::Time;

/// The response struct for Action SendGoal
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct ActionSendGoalResponse {
    pub accept: bool, // Accept the request or not
    pub timestamp: Time,
}

/// The request struct for getting ActionResult
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct ActionResultRequest {
    pub goal_id: [u8; 16],
}

/// The request struct for cancelling goal service
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct ActionCancelRequest {
    pub goal_id: [u8; 16],
    pub timestamp: Time,
}

/// The response struct for cancelling goal service
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct ActionCancelResponse {
    pub response_code: i8, // OK, REJECTED or INVALID_GOAL_ID
    pub goal_id: Vec<[u8; 16]>,
}
