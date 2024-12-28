use serde_derive::{Deserialize, Serialize};

use crate::{geometry_msgs, std_msgs};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CameraInfo {
    pub header: std_msgs::Header,
    pub height: u32,
    pub width: u32,
    pub distortion_model: String,
    pub d: Vec<f64>,
    pub k: [f64; 9],
    pub r: [f64; 9],
    pub p: [f64; 12],
    pub binning_x: u32,
    pub binning_y: u32,
    pub roi: RegionOfInterest,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Image {
    pub header: std_msgs::Header,
    pub height: u32,
    pub width: u32,
    pub encoding: String,
    pub is_bigendian: u8,
    pub step: u32,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct IMU {
    pub header: std_msgs::Header,
    pub orientation: geometry_msgs::Quaternion,
    pub orientation_covariance: [f64; 9],
    pub angular_velocity: geometry_msgs::Vector3,
    pub angular_velocity_covariance: [f64; 9],
    pub linear_acceleration: geometry_msgs::Vector3,
    pub linear_acceleration_covariance: [f64; 9],
}

pub mod nav_sat_fix {
    pub const COVARIANCE_TYPE_UNKNOWN: u8 = 0;
    pub const COVARIANCE_TYPE_APPROXIMATED: u8 = 1;
    pub const COVARIANCE_TYPE_DIAGONAL_KNOWN: u8 = 2;
    pub const COVARIANCE_TYPE_KNOWN: u8 = 3;
}
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct NavSatFix {
    pub header: std_msgs::Header,
    pub status: NavSatStatus,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub position_covariance: [f64; 9],
    pub position_covariance_type: u8,
}

pub mod nav_sat_status {
    pub const STATUS_NO_FIX: i8 = -1; // unable to fix position
    pub const STATUS_FIX: i8 = 0; // unaugmented fix
    pub const STATUS_SBAS_FIX: i8 = 1; // with satellite-based augmentation
    pub const STATUS_GBAS_FIX: i8 = 2; // with ground-based augmentation
    pub const SERVICE_GPS: u8 = 1;
    pub const SERVICE_GLONASS: u8 = 2;
    pub const SERVICE_COMPASS: u8 = 4; // includes BeiDou.
    pub const SERVICE_GALILEO: u8 = 8;
}
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct NavSatStatus {
    pub status: i8,
    pub service: u16,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct PointCloud2 {
    pub header: std_msgs::Header,
    pub height: u32,
    pub width: u32,
    pub fields: Vec<PointField>,
    pub is_bigendian: bool,
    pub point_step: u32,
    pub row_step: u32,
    pub data: Vec<u8>,
    pub is_dense: bool,
}

pub mod point_field {
    pub const INT8: u8 = 1;
    pub const UINT8: u8 = 2;
    pub const INT16: u8 = 3;
    pub const UINT16: u8 = 4;
    pub const INT32: u8 = 5;
    pub const UINT32: u8 = 6;
    pub const FLOAT32: u8 = 7;
    pub const FLOAT64: u8 = 8;
}
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct PointField {
    pub name: String,
    pub offset: u32,
    pub datatype: u8,
    pub count: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct RegionOfInterest {
    pub x_offset: u32,
    pub y_offset: u32,
    pub height: u32,
    pub width: u32,
    pub do_rectify: bool,
}
