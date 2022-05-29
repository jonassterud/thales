use super::*;
use bval::BVal;
use maplit::btreemap;

#[allow(dead_code)]
pub fn get_comparison_data() -> Vec<(&'static str, BVal)> {
    vec![
        ("i5944e", BVal::Number(5944)),
        ("i0e", BVal::Number(0)),
        ("i-50e", BVal::Number(-50)),
        ("3:abc", BVal::ByteString(vec![97, 98, 99])),
        ("0:", BVal::ByteString(vec![])),
        ("li573e3:abce", BVal::List(vec![BVal::Number(573), BVal::ByteString(vec![97, 98, 99])])),
        ("d3:abci573e3:cbai375ee", BVal::Dictionary(btreemap!(vec![97, 98, 99] => BVal::Number(573), vec![99, 98, 97] => BVal::Number(375)))),
    ]
}

#[test]
fn decode_bencode() {
    for (d, e) in get_comparison_data() {
        let left = decode::decode(d.as_bytes());
        let right = e;

        if left != right {
            panic!("failed decoding, {:?} != {:?}", left, right);
        }
    }
}

#[test]
fn encode_bencode() {
    for (d, e) in get_comparison_data() {
        let left = String::from_utf8(encode::encode(&e)).unwrap();
        let right = d;

        if left != right {
            panic!("failed encoding, {:?} != {:?}", left, right);
        }
    }
}

/*
#[allow(dead_code)]
fn test_decoding(encoded_data: Vec<&str>, decoded_data: Vec<BVal>) {
    for i in 0..encoded_data.len() {
        let left = decode::decode(encoded_data[i].as_bytes());
        let right = decoded_data.get(i).unwrap();

        if left != *right {
            panic!("failed decoding data, {:?} != {:?}", left, right);
        }
    }
}

#[test]
fn decode_integer() {
    let encoded_data = vec!["i5944e", "i0e", "i-50e"];
    let decoded_data = vec![BVal::Number(5944), BVal::Number(0), BVal::Number(-50)];
    test_decoding(encoded_data, decoded_data);
}

#[test]
fn decode_byte_string() {
    let encoded_data = vec!["3:abc", "0:"];
    let decoded_data = vec![BVal::ByteString("abc".as_bytes().to_vec()), BVal::ByteString(vec![])];
    test_decoding(encoded_data, decoded_data);
}

#[test]
fn decode_list() {
    let encoded_data = vec!["li573e3:abce"];
    let decoded_data = vec![BVal::List(vec![BVal::Number(573), BVal::ByteString("abc".as_bytes().to_vec())])];
    test_decoding(encoded_data, decoded_data);
}

#[test]
fn decode_dictionary() {
    let mut dict = std::collections::BTreeMap::<Vec<u8>, BVal>::new();
    dict.insert("abc".as_bytes().to_vec(), BVal::Number(573));
    dict.insert("cba".as_bytes().to_vec(), BVal::Number(375));

    let encoded_data = vec!["d3:abci573e3:cbai375ee"];
    let decoded_data = vec![BVal::Dictionary(dict)];
    test_decoding(encoded_data, decoded_data);
}

#[allow(dead_code)]
fn test_encoding(decoded_data: Vec<BVal>, encoded_data: Vec<&str>) {
    let left = encode::encode(decoded_data);
    let right = encoded_data.into_iter().collect::<String>().as_bytes();

    if left != right {
        panic!("failed encoding data, {:?} != {:?}", left, right);
    }
}

#[test]
fn encode_integer() {
    let decoded_data = vec![BVal::Number(5944), BVal::Number(0), BVal::Number(-50)];
    let encoded_data = vec!["i5944e", "i0e", "i-50e"];
    test_decoding(decoded_data, encoded_data);
}
*/