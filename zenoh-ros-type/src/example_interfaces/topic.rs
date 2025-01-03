use serde_derive::{Deserialize, Serialize};
use zenoh_ros_derive::ZBytesCdr;

macro_rules! define_multiarray_type {
    ($type_name:ident, $data_type:ty) => {
        #[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
        pub struct $type_name {
            pub data: $data_type,
        }

        // paste can help use to generate a struct name $[type_name]MultiArray
        paste::paste! {
            #[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
            pub struct [<$type_name MultiArray>] {
                pub layout: MultiArrayLayout,
                pub data: Vec<$data_type>,
            }
        }
    };
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct Bool {
    pub data: bool,
}

define_multiarray_type!(Byte, u8);

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct Char {
    pub data: char,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct Empty {}

define_multiarray_type!(Float32, f32);
define_multiarray_type!(Float64, f64);

define_multiarray_type!(Int16, i16);
define_multiarray_type!(Int32, i32);
define_multiarray_type!(Int64, i64);
define_multiarray_type!(Int8, i8);

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct MultiArrayDimension {
    pub label: String,
    pub size: u32,
    pub stride: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct MultiArrayLayout {
    pub dim: Vec<MultiArrayDimension>,
    pub data_offset: u32,
}

// To avoid conflict with the basic type in Rust, we add underscore
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, ZBytesCdr)]
pub struct String_ {
    pub data: String,
}

define_multiarray_type!(UInt16, u16);
define_multiarray_type!(UInt32, u32);
define_multiarray_type!(UInt64, u64);
define_multiarray_type!(UInt8, u8);
