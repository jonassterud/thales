use super::*;
use maplit::btreemap;
use val::Val;

#[allow(dead_code)]
pub fn get_comparison_data() -> Vec<(&'static str, Val)> {
    vec![
        ("i5944e", Val::Number(5944)),
        ("i0e", Val::Number(0)),
        ("i-50e", Val::Number(-50)),
        ("3:abc", Val::ByteString(vec![97, 98, 99])),
        ("0:", Val::ByteString(vec![])),
        (
            "li573e3:abce",
            Val::List(vec![Val::Number(573), Val::ByteString(vec![97, 98, 99])]),
        ),
        (
            "d3:abci573e3:cbai375ee",
            Val::Dictionary(
                btreemap!(vec![97, 98, 99] => Val::Number(573), vec![99, 98, 97] => Val::Number(375)),
            ),
        ),
    ]
}

#[test]
fn decode_bencode() {
    for (d, e) in get_comparison_data() {
        let left = decode::decode(d.as_bytes()).unwrap();
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
