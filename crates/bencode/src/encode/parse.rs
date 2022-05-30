use super::Val;
use std::collections::BTreeMap;

/// Encodes value as bencode.
///
/// # Arguments
///
/// * `content` - data to encode
pub fn any(content: &Val) -> Vec<u8> {
    match content {
        Val::Number(inner) => integer(inner),
        Val::ByteString(inner) => byte_string(inner),
        Val::List(inner) => list(inner),
        Val::Dictionary(inner) => dictionary(inner),
    }
}

/// Encodes number.
///
/// # Arguments
///
/// * `content` - number to encode
pub fn integer(content: &i64) -> Vec<u8> {
    let out = format!("i{}e", content.to_string());

    out.as_bytes().to_vec()
}

/// Encodes byte string.
///
/// # Arguments
///
/// * `content` - byte string to encode
pub fn byte_string(content: &Vec<u8>) -> Vec<u8> {
    let mut out = format!("{}:", content.len()).as_bytes().to_vec();

    out.append(&mut content.clone());

    out
}

/// Encodes list.
///
/// # Arguments
///
/// * `content` - list to encode
pub fn list(content: &Vec<Val>) -> Vec<u8> {
    let mut out: Vec<u8> = vec![];

    out.append(&mut "l".as_bytes().to_vec());
    for val in content {
        out.append(&mut any(val));
    }
    out.append(&mut "e".as_bytes().to_vec());

    out
}

// TODO: Assumes the content is in lexicographical order!
/// Encodes dictionary.
///
/// # Arguments
///
/// * `content` - dictionary to encode
pub fn dictionary(content: &BTreeMap<Vec<u8>, Val>) -> Vec<u8> {
    let mut out: Vec<u8> = vec![];

    out.append(&mut "d".as_bytes().to_vec());
    for (k, v) in content {
        out.append(&mut byte_string(k));
        out.append(&mut any(v));
    }
    out.append(&mut "e".as_bytes().to_vec());

    out
}
