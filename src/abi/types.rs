use std::fmt;
use std::str::FromStr;

use serde::Deserialize;
use serde_json::Value;
use strum::{EnumString, ToString};
use ethereum_types::{H160, U64, U256, U128};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, EnumString, ToString)]
pub enum Type {
    UnImplemented,
    #[strum(serialize = "int8")]
    Int8(i8),
    #[strum(serialize = "int32")]
    Int32(i32),
    #[strum(serialize = "int256")]
    Int256(U256),
    #[strum(serialize = "uint8")]
    Uint8(u8),
    #[strum(serialize = "uint32")]
    Uint32(u32),
    #[strum(serialize = "uint256")]
    Uint256(U256),
    #[strum(serialize = "bool")]
    Bool(bool),

    #[strum(serialize = "bytes32")]
    Bytes32(),

    #[strum(serialize = "address")]
    Address(H160),
    // TODO
    #[strum(serialize = "address[]")]
    Addresses(H160),
    // TODO
    #[strum(serialize = "uint256[]")]
    Uints256(U128),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub internal_type: Option<Type>,
    pub name: String,
    pub _type: Type,
}

impl Param {
    pub fn parse(raw_param: &serde_json::Value) -> Self {
        match raw_param {
            Value::Object(obj) => {
                Self {
                    internal_type: Self::internal_type(&obj["internalType"]),
                    name: Self::name(&obj["name"]),
                    _type: Self::typ(&obj["type"]),
                }
            }
            _ => panic!("invalid param")
        }
    }

    fn internal_type(raw_type: &serde_json::Value) -> Option<Type> {
        match raw_type {
            Value::String(typ) => {
                match Type::from_str(typ.as_str()) {
                    Ok(v) => Some(v),
                    Err(_) => Some(Type::UnImplemented)
                }
            }
            _ => None
        }
    }

    fn typ(raw_type: &serde_json::Value) -> Type {
        match raw_type {
            Value::String(typ) => {
                match Type::from_str(typ.as_str()) {
                    Ok(v) => v,
                    Err(_) => Type::UnImplemented
                }
            }
            _ => panic!("missing type")
        }
    }

    fn name(raw_name: &serde_json::Value) -> String {
        match raw_name {
            Value::String(name) => name.clone(),
            _ => "".to_string(),
        }
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.internal_type {
            Some(v) => write!(f, "{{ type: {}, name: {}, internalType: {} }}", self._type.to_string(), self.name.to_string(), v.to_string()),
            None => write!(f, "{{ type: {}, name: {} }}", self._type.to_string(), self.name.to_string()),
        }
    }
}

impl Type {
    pub fn address_from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() <= 20, "Byte array doesn't fit into 160 bits");
        Self::Address(H160::from_slice(bytes))
    }

    pub fn u256_from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() <= 8, "Byte array doesn't fit into 64 bits");
        Self::Uint256(U256::from(bytes))
    }

    // pub fn u128_from_bytes(bytes: &[u8]) -> Self {
    //     assert!(bytes.len() <= 16, "Byte array doesn't fit into 128 bits");
    //     Self::U128(U128::from(bytes))
    // }
    //
    // pub fn u256_from_bytes(bytes: &[u8]) -> Self {
    //     assert!(bytes.len() <= 32, "Byte array doesn't fit into 256 bits");
    //     Self::U256(U256::from(bytes))
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_address_from_bytes() {
        let address = Type::address_from_bytes(&[0x30, 0xE7, 0xd7, 0xFf, 0xF8, 0x5C, 0x8d, 0x0E, 0x77, 0x51, 0x40, 0xb1, 0xaD, 0x93,
                                       0xC2, 0x30, 0xD5, 0x59, 0x52, 0x07,]);
        println!("{:?}",address);
    }
}
