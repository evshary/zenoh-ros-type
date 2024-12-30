use serde_derive::{Deserialize, Serialize};
use zenoh_ros_derive::ZBytesCdr;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct FibonacciSendGoal {
    pub goal_id: [u8; 16],
    pub goal: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct FibonacciResult {
    // Refer to action status
    pub status: i8,
    pub sequence: Vec<i32>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct FibonacciFeedback {
    pub goal_id: [u8; 16],
    pub sequence: Vec<i32>,
}
