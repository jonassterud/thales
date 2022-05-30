use std::collections::BTreeMap;
use anyhow::{anyhow, Result};

/// Enum to represent Bencoded data values.
#[derive(Debug, PartialEq, Clone)]
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

impl Val {
    pub fn as_number(&self) -> Result<i64> {
        if let Val::Number(out) = self {
            Ok(out.clone())
        } else {
            Err(anyhow!("{:?} is not a number", self))
        }
    }

    pub fn as_byte_string(&self) -> Result<Vec<u8>> {
        if let Val::ByteString(out) = self {
            Ok(out.clone())
        } else {
            Err(anyhow!("{:?} is not a byte string", self))
        }
    }

    pub fn as_list(&self) -> Result<Vec<Val>> {
        if let Val::List(out) = self {
            Ok(out.clone())
        } else {
            Err(anyhow!("{:?} is not a list", self))
        }
    }

    pub fn as_dictionary(&self) -> Result<BTreeMap<Vec<u8>, Val>> {
        if let Val::Dictionary(out) = self {
            Ok(out.clone())
        } else {
            Err(anyhow!("{:?} is not a dictionary", self))
        }
    }
}