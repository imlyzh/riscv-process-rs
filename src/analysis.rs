pub mod range;

use std::collections::HashMap;

use crate::node::Node;
use self::range::*;


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