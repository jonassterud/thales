use super::*;
use val::Val;

#[allow(dead_code)]
fn test_decoding(encoded_data: Vec<&str>, decoded_data: Vec<Val>) {
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
    let decoded_data = vec![Val::Number(5944), Val::Number(0), Val::Number(-50)];
    test_decoding(encoded_data, decoded_data);
}

#[test]
fn decode_byte_string() {
    let encoded_data = vec!["3:abc", "0:"];
    let decoded_data = vec![Val::ByteString("abc".as_bytes().to_vec()), Val::ByteString(vec![])];
    test_decoding(encoded_data, decoded_data);
}

#[test]
fn decode_list() {
    let encoded_data = vec!["li573e3:abce"];
    let decoded_data = vec![Val::List(vec![Val::Number(573), Val::ByteString("abc".as_bytes().to_vec())])];
    test_decoding(encoded_data, decoded_data);
}

#[test]
fn decode_dictionary() {
    let mut dict = std::collections::BTreeMap::<Vec<u8>, Val>::new();
    dict.insert("abc".as_bytes().to_vec(), Val::Number(573));
    dict.insert("cba".as_bytes().to_vec(), Val::Number(375));

    let encoded_data = vec!["d3:abci573e3:cbai375ee"];
    let decoded_data = vec![Val::Dict(dict)];
    test_decoding(encoded_data, decoded_data);
}