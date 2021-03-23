use serde_json::Value;
use crate::abi::types::Param;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub _type: String,
    pub name: String,
    pub inputs: Vec<Param>,
    pub outputs: Vec<Param>,
    pub state_mutability: Option<StateMutability>,
    pub payable: Option<bool>,
    pub constant: Option<bool>,
}

impl Function {
    pub fn parse(raw_func: &serde_json::Value) -> Self {
        return Self {
            _type: "function".to_string(),
            name: Self::name(raw_func),
            inputs: Self::inputs(&raw_func),
            outputs: Self::outputs(&raw_func),
            state_mutability: StateMutability::parse(&raw_func),
            payable: Self::payable(&raw_func),
            constant: Self::constant(&raw_func),
        };
    }

    fn inputs(raw_func: &serde_json::Value) -> Vec<Param> {
        match &raw_func["inputs"] {
            Value::Array(v) => {
                let mut res = vec![];
                for input in v {
                    res.push(Param::parse(&input));
                }
                res
            },
            _ => panic!("ABI input is not properly formatted")
        }
    }

    fn outputs(raw_func: &serde_json::Value) -> Vec<Param> {
        match &raw_func["outputs"] {
            Value::Array(v) => {
                let mut res = vec![];
                for input in v {
                    res.push(Param::parse(&input));
                }
                res
            },
            _ => panic!("ABI output is not properly formatted")
        }
    }

    fn payable(raw_func: &serde_json::Value) -> Option<bool> {
        let payable = &raw_func["payable"];
        match payable {
            Value::Bool(val) => Some(*val),
            _ => None,
        }
    }

    fn constant(raw_func: &serde_json::Value) -> Option<bool> {
        let raw_constant = &raw_func["constant"];
        match raw_constant {
            Value::Bool(val) => Some(*val),
            _ => None
        }
    }

    fn name(raw_func: &serde_json::Value) -> String {
        let raw_name = &raw_func["name"];
        match raw_name {
            Value::String(name) => name.clone(),
            _ => "".to_string(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StateMutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

impl StateMutability {
    pub fn parse(raw_func: &serde_json::Value) -> Option<Self> {
        let raw_state_mutability = &raw_func["stateMutability"];
        match raw_state_mutability {
            Value::String(val) => {
                match val.as_str() {
                    "pure" => Some(StateMutability::Pure),
                    "view" => Some(StateMutability::View),
                    "nonpayable" => Some(StateMutability::NonPayable),
                    "payable" => Some(StateMutability::Payable),
                    _ => {
                        panic!("TODO");
                    }
                }
            }
            _ => None,
        }
    }
}

