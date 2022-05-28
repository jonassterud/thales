use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Val {
    Number(i64),
    ByteString(Vec<u8>),
    List(Vec<Val>),
    Dict(BTreeMap<Vec<u8>, Val>),
}