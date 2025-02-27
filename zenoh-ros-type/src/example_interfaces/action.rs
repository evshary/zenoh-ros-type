use crate::unique_identifier_msgs::UUID;
use serde_derive::{Deserialize, Serialize};
use zenoh_ros_derive::ZBytesCdr;

// TODO: Find a way to avoid using UUID
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct FibonacciSendGoal {
    pub goal_id: UUID,
    pub goal: i32,
}

// TODO: Find a way to avoid using status
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct FibonacciResult {
    // Refer to action status
    pub status: i8,
    pub sequence: Vec<i32>,
}

// TODO: Find a way to avoid using UUID
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct FibonacciFeedback {
    pub goal_id: UUID,
    pub sequence: Vec<i32>,
}
