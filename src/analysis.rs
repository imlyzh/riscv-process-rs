use std::collections::HashMap;

use crate::node::Node;


pub type RegisterType = SimpleDataType;

#[derive(Debug, Clone, Copy)]
pub enum SimpleDataType {
    Unknown,
    Zero,
    NonZero,
    Bool,
    Pointer(Pointer),
    I32(I32),
    U32(U32),
    // F32(F32),
}

#[derive(Debug, Clone, Copy)]
pub struct Pointer(pub u32, pub u32);

impl Pointer {
    pub fn new() -> Self {
        Self(u32::MIN, u32::MAX)
    }

    pub fn from(i: u32) -> Self {
        Self(i, i)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct I32(pub i32, pub i32);

impl I32 {
    pub fn new() -> Self {
        Self(i32::MIN, i32::MAX)
    }

    pub fn from(i: i32) -> Self {
        Self(i, i)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct U32(pub u32, pub u32);

impl U32 {
    pub fn new() -> Self {
        Self(u32::MIN, u32::MAX)
    }

    pub fn from(i: u32) -> Self {
        Self(i, i)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct F32(pub f32, pub f32);

impl F32 {
    pub fn new() -> Self {
        Self(f32::MIN, f32::MAX)
    }

    pub fn from(i: f32) -> Self {
        Self(i, i)
    }
}

#[derive(Debug, Clone)]
pub struct MemRecord(pub HashMap<String, MemItem>);


#[derive(Debug, Clone)]
pub enum MemItem {
    Inst(Node),
    Data(DataBlock)
}


#[derive(Debug, Clone)]
pub struct DataBlock {
    // meta info
    datas: Vec<Data>
}

#[derive(Debug, Clone)]
pub struct Data(pub Vec<u8>, pub SimpleDataType);