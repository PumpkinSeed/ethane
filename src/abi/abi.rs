use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub use crate::abi::function::{Function, StateMutability};
use crate::abi::types::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct Abi {
    pub functions: Vec<Function>
}

#[allow(dead_code)]
impl Abi {
    pub fn new() -> Self {
        Abi { functions: vec![] }
    }

    pub fn parse(&mut self, path_to_abi: &Path) -> Result<Vec<Function>, String> {
        let file = File::open(path_to_abi).map_err(|e| format!("Couldn't open file: {}", e))?;
        let reader = BufReader::new(file);
        let functions: serde_json::Value =
            serde_json::from_reader(reader).map_err(|e| format!("Couldn't parse json: {}", e))?;

        let mut i: usize = 0;
        while functions[i] != serde_json::Value::Null {
            if functions[i]["type"] == "function" {
                self.functions.push(Function::parse(&functions[i]));
            }
            i += 1;
        }

        Ok(self.functions.clone())
    }

    pub fn encode(&mut self, function_name: &str, arguments: Vec<Type>) {
        if let Some(f) = self.find_function_by_name(function_name) {
            println!("{:?}",f)
        } else {
            panic!("unable to find function")
        }
    }

    fn find_function_by_name(&self, function_name: &str) -> Option<Function> {
        for f in &self.functions {
            if f.name == function_name {
                return Some(f.clone())
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        // let path = Path::new("src/abi/abi.json");
        let path = Path::new("test-helper/src/fixtures/TestABI.json");


        let mut abi = Abi::new();
        println!("{:?}", abi);
        let f = abi.parse(path).expect("unable to parse abi");
        println!("{:?}",f);
        println!("{:?}",f[0].outputs[0].to_string());

        abi.encode("WETH",vec![]);
    }
}
