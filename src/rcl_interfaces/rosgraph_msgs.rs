use serde_derive::{Deserialize, Serialize};

use crate::builtin_interfaces::Time;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Clock {
    pub clock: Time,
}
