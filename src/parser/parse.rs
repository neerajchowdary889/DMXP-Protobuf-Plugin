use crate::parser::parser;
use crate::ast::structs::ProtoFile;
use anyhow::{Error, Result};
use crate::utils::LoadFile;

/// Parse a protobuf file from disk into an AST
/// 
/// This is a convenience function that reads a protobuf file from the filesystem
/// and parses it into a structured AST representation.
/// 
/// # Arguments
/// * `file_path` - Path to the protobuf file to parse
/// 
/// # Returns
/// * `Result<ProtoFile>` - The parsed AST or an error if parsing fails
/// 
/// # Errors
/// Returns an error if the file cannot be read or if parsing fails
pub fn parse_proto_file(file_path: &str) -> Result<ProtoFile, Error> {
    let content = LoadFile::LoadFile(file_path)?;
    let mut parser = parser::ProtoParser::new(content);
    parser.parse()
}
