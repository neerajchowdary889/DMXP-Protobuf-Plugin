mod ast;
mod parser;
mod utils;
mod templateGen;

use crate::parser::parse_proto_file;
use crate::templateGen::{TemplateGenerator, Language};
use anyhow::Result;

fn main() -> Result<()> {
    println!("DMXP Protobuf Plugin - Parsing test.proto");
    
    // Load the protofile to string from the file
    let proto_file_string = utils::LoadFile::LoadFile("test.proto")?;
    
    // Parse the test.proto file into AST
    let proto_file = parse_proto_file("test.proto")?;
    
    // Display the parsed AST
    println!("\n=== PARSED PROTOBUF FILE ===");
    println!("Package: {}", proto_file.package);
    println!("Syntax: {}", proto_file.syntax);
    
    println!("\n=== MESSAGES ===");
    for message in &proto_file.messages {
        println!("Message: {}", message.name);
        println!("  Fields: {}", message.fields.len());
        for field in &message.fields {
            println!("    - {}: {:?} (number: {})", field.name, field.field_type, field.number);
        }
        
        if let Some(dmxp_opts) = &message.dmxp_options {
            if let Some(channel) = &dmxp_opts.channel {
                println!("  DMXP Channel: {}", channel);
            }
        }
        println!();
    }
    
    println!("\n=== SERVICES ===");
    for service in &proto_file.services {
        println!("Service: {}", service.name);
        println!("  Methods: {}", service.methods.len());
        for method in &service.methods {
            println!("    - {}: {} -> {}", method.name, method.input_type, method.output_type);
        }
        
        if let Some(dmxp_opts) = &service.dmxp_options {
            println!("  DMXP Channels: {:?}", dmxp_opts.channels);
        }
        println!();
    }
    
    println!("\n=== ENUMS ===");
    for enum_def in &proto_file.enums {
        println!("Enum: {}", enum_def.name);
        println!("  Values: {}", enum_def.values.len());
        for value in &enum_def.values {
            println!("    - {} = {}", value.name, value.number);
        }
        println!();
    }
    
    println!("\n=== DMXP CHANNELS ===");
    for channel in &proto_file.dmxp_channels {
        println!("Channel: {} (type: {}, direction: {:?})", 
                channel.name, channel.message_type, channel.direction);
    }
    
    println!("\nAST parsing completed successfully!");
    
    // Generate code for different languages
    println!("\n=== GENERATING CODE ===");
    
    // Generate Rust code
    println!("\n--- Generating Rust Code ---");
    let rust_generator = TemplateGenerator::new(Language::Rust);
    let rust_code = rust_generator.generate(&proto_file)?;
    println!("Generated {} lines of Rust code", rust_code.lines().count());
    
    // Generate Go code
    println!("\n--- Generating Go Code ---");
    let go_generator = TemplateGenerator::new(Language::Go);
    let go_code = go_generator.generate(&proto_file)?;
    println!("Generated {} lines of Go code", go_code.lines().count());
    
    // Save generated code to files
    std::fs::write("generated_rust.rs", rust_code)?;
    std::fs::write("generated_go.go", go_code)?;
    
    println!("\nGenerated code saved to:");
    println!("- generated_rust.rs");
    println!("- generated_go.go");
    
    println!("\nTemplate generation completed successfully!");
    Ok(())
}
