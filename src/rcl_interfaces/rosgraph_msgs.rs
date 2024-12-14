use serde_derive::{Deserialize, Serialize};

use crate::builtin_interfaces::Time;

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Clock {
    pub clock: Time,
}
