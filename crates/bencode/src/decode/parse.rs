use super::Val;
use std::collections::BTreeMap;
use std::iter::Peekable;
use std::slice::Iter;

pub fn any(content: &mut Peekable<Iter<u8>>) -> Option<Val> {
    while let Some(byte) = content.peek() {
        match byte {
            105 => return Some(integer(content)),
            48..=57 => return Some(byte_string(content)),
            108 => return Some(list(content)),
            100 => return Some(dictionary(content)),
            _ => {}
        }
    }

    None
}

pub fn integer(content: &mut Peekable<Iter<u8>>) -> Val {
    if **content.peek().unwrap() != 105 {
        panic!("missing 'i'");
    } else {
        content.next();
    }

    let mut int_temp: Vec<char> = vec![];
    let int: i64;
    
    while let Some(byte) = content.peek() {
        match byte {
            45 /* - */ => {
                if int_temp.len() != 0 {
                    panic!("unexpected byte");
                }

                int_temp.push(*content.next().unwrap() as char);
            },
            48..=57 /* 0-9 */ => {
                int_temp.push(*content.next().unwrap() as char);
            },
            101 /* e */ => {
                content.next();
                break;
            },
            _ => panic!("unexpected byte"),
        }
    }

    int = int_temp.iter().collect::<String>().parse::<i64>().unwrap();
    
    Val::Number(int)

}

pub fn byte_string(content: &mut Peekable<Iter<u8>>) -> Val {
    let mut bs_len_temp: Vec<char> = vec![];
    let mut bs_len: i64 = 0;
    let mut bs: Vec<u8> = vec![];
   
    while let Some(byte) = content.peek() {
        match byte {
            48..=57 /* 0-9 */ => {
                bs_len_temp.push(*content.next().unwrap() as char);
            },
            58 /* : */ => {
                content.next();
                bs_len = bs_len_temp.iter().collect::<String>().parse::<i64>().unwrap();
                break;
            },
            _ => panic!("unexpected byte"),
        };   
    }

    for _ in 0..bs_len {
        bs.push(*content.next().unwrap());
    }

    Val::ByteString(bs)
}

pub fn list(content: &mut Peekable<Iter<u8>>) -> Val {
    let mut list: Vec<Val> = vec![];

    if **content.peek().unwrap() != 108 /* l */ {
        panic!("missing 'l'");
    } else {
        content.next();
    }

    while **content.peek().unwrap() != 101 /* e */ {
        list.push(any(content).unwrap());
    }

    Val::List(list)
}

pub fn dictionary(content: &mut Peekable<Iter<u8>>) -> Val {
    let mut dict: BTreeMap<Vec<u8>, Val> = BTreeMap::new();

    if **content.peek().unwrap() != 100 /* d */ {
        panic!("missing 'd'");
    } else {
        content.next();
    }

    while **content.peek().unwrap() != 101 /* e */ {
        let key_temp = any(content).unwrap();

        if let Val::ByteString(key) = key_temp {
            let value = any(content).unwrap();
            dict.insert(key, value);
        } else {
            panic!("key is not a byte string")
        }
    }

    Val::Dict(dict)
}