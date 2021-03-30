use std::fmt;
use std::str::FromStr;

// use ethereum_types::{Address, H160, U128, U256, U64};
use ethereum_types::Address;
use serde::Deserialize;
// use serde_json::Value;
use strum::{EnumString, ToString};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, EnumString, ToString)]
pub enum Type {
    UnImplemented,
    #[strum(serialize = "int8")]
    Int8(i8),
    #[strum(serialize = "int16")]
    Int16(i16),
    #[strum(serialize = "address")]
    Address(Address),
}

fn encode_type(typ: &Type) -> Vec<u8> {
    match *typ {
        Type::Int8(i8) => {
            let mut padded = [0u8; 32];
            padded[31] = i8 as u8;
            return padded.to_vec();
        }
        Type::Int16(i16) => {
            let mut padded = [0u8; 32];
            padded[30..].copy_from_slice(&i16.to_be_bytes());
            return padded.to_vec();
        }
        Type::Address(ref address) => {
            let mut padded = [0u8; 32];
            padded[12..].copy_from_slice(address.as_ref());
            return padded.to_vec();
        }
        Type::UnImplemented => panic!("type not implemented"),
    }
}

fn decode_type(typ: Type, raw: Vec<u8>) -> Type {
    return match typ {
        Type::UnImplemented => panic!("Err"),
        Type::Int8(_) => panic!("Err"),
        Type::Int16(_) => panic!("Err"),
        Type::Address(_) => Type::Address(Address::from_slice(&raw[12..])),
    };
}

#[cfg(test)]
mod tests {
    use crate::abi2::types::{decode_type, encode_type, Type};
    use ethereum_types::{H160,Address};
    use hex_literal::hex;
    use std::str::FromStr;

    #[test]
    fn encode_address() {
        let address = Type::Address([0x11u8; 20].into());
        let encoded = encode_type(&address);
        let expected = hex!("0000000000000000000000001111111111111111111111111111111111111111");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn decode_address() {
        let address = Type::Address(H160::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap());
        let encoded = encode_type(&address);
        let act_type = decode_type(Type::Address(Address::zero()), encoded.clone());
        let exp_typ = Type::Address(H160::from_str("0x95eDA452256C1190947f9ba1fD19422f0120858a").unwrap());
        println!("{:?}", act_type);
        assert_eq!(act_type, exp_typ);
    }

    #[test]
    fn encode_i8() {
        let int8 = Type::Int8(5);
        let encoded = encode_type(&int8);
        let expected = hex!("0000000000000000000000000000000000000000000000000000000000000005");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn encode_i16() {
        let int16 = Type::Int16(32765);
        let encoded = encode_type(&int16);
        let expected = hex!("0000000000000000000000000000000000000000000000000000000000007FFD");
        assert_eq!(encoded, expected);
    }

    #[test]
    fn encode_i16_2() {
        let int16 = Type::Int16(0);
        let encoded = encode_type(&int16);
        let expected = hex!("0000000000000000000000000000000000000000000000000000000000000000");
        assert_eq!(encoded, expected);
    }
}
