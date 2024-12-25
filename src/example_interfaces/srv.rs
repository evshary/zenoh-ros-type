use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct AddTwoIntsRequest {
    pub a: i64,
    pub b: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct AddTwoIntsReply {
    pub sum: i64,
}
