use DMXP_Protobuf_Plugin::ast::AstBuilder;
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

#[test]
fn test_parse_proto_with_dmxp_options() {
    println!("Parsing test.proto");

    let content = LoadFile::LoadFile("test.proto").expect("Failed to load test.proto");
    let mut parser = ProtoParser::new(content);
    
    // Parse the proto file
    let result = parser.parse();
    assert!(result.is_ok(), "Parsing failed: {:?}", result.err());
    
    // Use the parsed AST returned by the parser
    let ast = result.unwrap();
    
    println!("AST generated successfully: {:#?}", ast);
    // Verify that we have the expected messages with DMXP options
    assert!(!ast.messages.is_empty(), "No messages found in the AST");
    
    // Check UserData message DMXP options
    let user_data = ast.messages.iter()
        .find(|m| m.name == "UserData")
        .expect("UserData message not found");
        
    assert!(user_data.dmxp_options.is_some(), "DMXP options not found for UserData");
    let dmxp_options = user_data.dmxp_options.as_ref().unwrap();
    assert_eq!(dmxp_options.channel, Some("user_updates".to_string()));
    
    // Check OrderData message DMXP options
    let order_data = ast.messages.iter()
        .find(|m| m.name == "OrderData")
        .expect("OrderData message not found");
        
    assert!(order_data.dmxp_options.is_some(), "DMXP options not found for OrderData");
    let order_dmxp = order_data.dmxp_options.as_ref().unwrap();
    assert_eq!(order_dmxp.channel, Some("order_events".to_string()));
    
    println!("AST generated successfully: {:#?}", ast);
}