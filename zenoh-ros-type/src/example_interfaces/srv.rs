use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct AddTwoIntsRequest {
    pub a: i64,
    pub b: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct AddTwoIntsReply {
    pub sum: i64,
}
