use std::collections::BTreeMap;

/// Enum to represent Bencoded data values.
#[derive(Debug, PartialEq)]
pub enum Val {
    /// Integer
    Number(i64),
    /// Byte string
    ByteString(Vec<u8>),
    /// List of values
    List(Vec<Val>),
    /// Dictionary of values
    Dictionary(BTreeMap<Vec<u8>, Val>),
}
