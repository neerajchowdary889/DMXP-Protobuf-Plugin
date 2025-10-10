pub mod parser;
pub mod parse;
pub mod helpers;

// Re-export the main parsing function for easy access
pub use parse::parse_proto_file;