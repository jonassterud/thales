use super::Val;
use std::collections::BTreeMap;
use std::iter::Peekable;
use std::slice::Iter;
use anyhow::{anyhow, Result};

/// Figures out what the bencoded data is, and calls the appropriate decode function.
///
/// # Arguments
///
/// * `content` - data to decode
pub fn any(content: &mut Peekable<Iter<u8>>) -> Result<Val> {
    // Call appropiate parser function based on the first byte
    if let Some(byte) = content.peek() {
        match byte {
            105 => integer(content),
            48..=57 => byte_string(content),
            108 => list(content),
            100 => dictionary(content),
            _ => Err(anyhow!("unexpected data in content (any): {}", **byte as char))
        }
    } else {
        Err(anyhow!("content is empty"))
    }
}

/// Decode bencoded integer.
/// An integer is bencoded like this: `i<integer>e`.
///
/// # Arguments
///
/// * `content` - data to decode
pub fn integer(content: &mut Peekable<Iter<u8>>) -> Result<Val> {
    if content.next().unwrap() != &105 {
        return Err(anyhow!("missing 'i' to mark start of integer"));
    }

    let mut buffer: Vec<char> = vec![];
    while let Some(byte) = content.next() {
        match byte {
            45 /* - */ => {
                if !buffer.is_empty() {
                    return Err(anyhow!("unexpected '-', only allowed at start of integer"));
                } else {
                    buffer.push(*byte as char);
                }
            },
            48..=57 /* 0-9 */ => buffer.push(*byte as char),
            101 /* e */ => break,
            _ => return Err(anyhow!("unexpected data in content (integer): {}", *byte as char)),
        }
    }

    let out = buffer.iter().collect::<String>().parse::<i64>()?;

    Ok(Val::Number(out))
}

/// Decode bencoded byte string.
/// A byte string is bencoded like this: `<length_in_bytes>:<bytes>`.
///
/// # Arguments
///
/// * `content` - data to decode
pub fn byte_string(content: &mut Peekable<Iter<u8>>) -> Result<Val> {
    let mut len_buffer: Vec<char> = vec![];

    while let Some(byte) = content.next() {
        match byte {
            48..=57 /* 0-9 */ => len_buffer.push(*byte as char),
            58 /* : */ => {
                let len = len_buffer.iter().collect::<String>().parse::<i64>()?;
                let mut out: Vec<u8> = vec![];

                for _ in 0..len {
                    out.push(*content.next().unwrap());
                }
            
                return Ok(Val::ByteString(out))
            },
            _ => return Err(anyhow!("unexpected data in content (byte string): {}", *byte as char)),
        };
    }

    Err(anyhow!("no data in content"))
}

/// Decode bencoded list.
/// A list is bencoded like this: `l<content>e`.
///
/// # Arguments
///
/// * `content` - data to decode
pub fn list(content: &mut Peekable<Iter<u8>>) -> Result<Val> {
    if content.next().unwrap() != &108 /* l */ {
        return Err(anyhow!("missing 'l' to mark start of list"));
    }

    let mut out: Vec<Val> = vec![];

    while **content.peek().unwrap() != 101 /* e */ {
        out.push(any(content)?);
    }

    // Skip 'e'
    content.next();

    Ok(Val::List(out))
}

/// Decode bencoded dictionary.
/// A dictionary is encoded like this: `d<key_as_byte_string><value>(etc)e`.
///
/// # Arguments
///
/// * `content` - data to decode
pub fn dictionary(content: &mut Peekable<Iter<u8>>) -> Result<Val> {
    if content.next().unwrap() != &100 {
        return Err(anyhow!("missing 'd' to mark start of dictionary"));
    }

    let mut out: BTreeMap<Vec<u8>, Val> = BTreeMap::new();

    while **content.peek().unwrap() != 101 /* e */ {
        if let Val::ByteString(key) = any(content)? {
            out.insert(key, any(content)?);
        } else {
            return Err(anyhow!("key in dictionary is not a byte string"));
        }
    }

    // Skip 'e'
    content.next();

    Ok(Val::Dictionary(out))
}
