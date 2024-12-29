use serde_derive::{Deserialize, Serialize};

use crate::service::ServiceHeader;

pub mod response_status {
    pub const SUCCESS: u32 = 1;
    pub const IGNORED: u32 = 2;
    pub const WARN: u32 = 3;
    pub const ERROR: u32 = 4;
}
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ResponseStatus {
    pub code: u32,
    pub message: String,
}

// -----service-----

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct EngageRequest {
    pub mode: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct EngageResponse {
    pub status: ResponseStatus,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct RawEngageRequest {
    pub header: ServiceHeader,
    pub mode: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct RawEngageResponse {
    pub header: ServiceHeader,
    pub status: ResponseStatus,
}
