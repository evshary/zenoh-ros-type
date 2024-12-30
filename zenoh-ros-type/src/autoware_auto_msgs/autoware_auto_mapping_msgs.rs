use serde_derive::{Deserialize, Serialize};
use zenoh_ros_derive::ZBytesCdr;

use crate::std_msgs::Header;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct HADMapBin {
    pub header: Header,
    pub map_format: u8,
    pub format_version: String,
    pub map_version: String,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct MapPrimitive {
    pub id: i64,
    pub primitive_type: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct HADMapSegment {
    pub primitives: Vec<MapPrimitive>,
    pub preferred_primitive_id: i64,
}
