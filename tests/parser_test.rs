use DMXP_Protobuf_Plugin::parser::parser::ProtoParser;
use DMXP_Protobuf_Plugin::utils::LoadFile;


#[cfg(test)]
mod tests {
    use super::*;

    mod proto_parser_tests {
        use super::*;

        #[test]
        fn test_new_empty() {
            let parser = ProtoParser::new(String::from(""));
            assert_eq!(parser.current_line, 0);
            println!("Printing the full object: {:?}", parser);
        }

        #[test]
        fn test_new_with_content() {
            let parser = ProtoParser::new(String::from("syntax = \"proto3\";"));
            assert_eq!(parser.current_line, 0);
            println!("Printing the full object: {:?}", parser);
        }

        #[test]
        fn test_new_with_parsed_content() {
            let content = LoadFile::LoadFile("test.proto").unwrap();
            let parser = ProtoParser::new(content);
            assert_eq!(parser.current_line, 0);
            println!("Printing the full object: {:?}", parser);
        }
    }
}