use serde_derive::{Deserialize, Serialize};

use crate::std_msgs::Header;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct HADMapBin {
    pub header: Header,
    pub map_format: u8,
    pub format_version: String,
    pub map_version: String,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct MapPrimitive {
    pub id: i64,
    pub primitive_type: String,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct HADMapSegment {
    pub primitives: Vec<MapPrimitive>,
    pub preferred_primitive_id: i64,
}
