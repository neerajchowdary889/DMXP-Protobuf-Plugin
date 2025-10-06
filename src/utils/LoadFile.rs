#![allow(dead_code)]
#![allow(unused_imports)]

use std::fs::File;
use std::io::Read;
use anyhow::{Error, Result};


pub fn LoadFile(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_file() {
        println!("Loading file: test.proto");
        let result = LoadFile("/Users/neeraj/CodeSection/DMXP-Protobuf-Plugin/test.proto");
        println!("Result: {:?}", result);
        assert!(result.is_ok());
    }
}