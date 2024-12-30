use serde_derive::{Deserialize, Serialize};
use zenoh_ros_derive::ZBytesCdr;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct AddTwoIntsRequest {
    pub a: i64,
    pub b: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct AddTwoIntsReply {
    pub sum: i64,
}
