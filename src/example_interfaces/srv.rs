use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
struct AddTwoIntsRequest {
    a: i64,
    b: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
struct AddTwoIntsReply {
    sum: i64,
}
