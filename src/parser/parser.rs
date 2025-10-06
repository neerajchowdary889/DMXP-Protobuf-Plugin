use crate::ast::*;
use anyhow::{Result};

/// Protobuf parser that converts .proto files to AST
/// 
/// This parser implements a line-by-line parsing approach for protobuf files,
/// extracting messages, services, enums, and DMXP-specific options into a structured AST.
#[derive(Debug)]
pub struct ProtoParser {
    /// Raw content of the protobuf file
    pub content: String,
    /// Current character position in the content (unused in line-based approach)
    pub position: usize,
    /// Lines of the protobuf file for easier processing
    pub lines: Vec<String>,
    /// Current line being processed
    pub current_line: usize,
}

impl ProtoParser {
    /// Create a new parser from file content
    /// 
    /// # Arguments
    /// * `content` - The raw protobuf file content as a string
    /// 
    /// # Returns
    /// A new ProtoParser instance ready to parse the content
    pub fn new(content: String) -> Self {
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        Self {
            content,
            position: 0,
            lines,
            current_line: 0,
        }
    }

    /// Parse the protobuf content into an AST
    /// 
    /// This is the main entry point for parsing. It processes the entire protobuf file
    /// and builds a structured AST representation containing all messages, services,
    /// enums, and DMXP-specific options.
    /// 
    /// # Returns
    /// * `Result<ProtoFile>` - The parsed AST or an error if parsing fails
    /// 
    /// # Errors
    /// Returns an error if the protobuf syntax is invalid or if parsing fails
    pub fn parse(&mut self) -> Result<ProtoFile> {
        let mut builder = AstBuilder::new();
        
        // Process each line of the protobuf file
        while self.current_line < self.lines.len() {
            let line = self.lines[self.current_line].trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with("//") {
                self.current_line += 1;
                continue;
            }
            
            // Route to appropriate parser based on line content
            if line.starts_with("syntax") {
                self.parse_syntax(&mut builder)?;
            } else if line.starts_with("package") {
                self.parse_package(&mut builder)?;
            } else if line.starts_with("message") {
                self.parse_message(&mut builder)?;
            } else if line.starts_with("service") {
                self.parse_service(&mut builder)?;
            } else if line.starts_with("enum") {
                self.parse_enum(&mut builder)?;
            }
            
            self.current_line += 1;
        }
        
        Ok(builder.build())
    }

    /// Parse the syntax declaration (e.g., "syntax = \"proto3\";")
    /// 
    /// # Arguments
    /// * `builder` - The AST builder to add the syntax information to
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if parsing fails
    fn parse_syntax(&mut self, builder: &mut AstBuilder) -> Result<()> {
        let line = self.lines[self.current_line].trim();
        
        if let Some(rest) = line.strip_prefix("syntax") {
            let rest = rest.trim_start_matches(|c: char| c.is_whitespace() || c == '=').trim_start();
            let syntax = rest.trim_end_matches(';').trim().trim_matches('\"');
            if !syntax.is_empty() {
                builder.set_syntax(syntax.to_string());
            }
        }
        
        Ok(())
    }

    /// Parse the package declaration (e.g., "package com.example;")
    /// 
    /// # Arguments
    /// * `builder` - The AST builder to add the package information to
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if parsing fails
    fn parse_package(&mut self, builder: &mut AstBuilder) -> Result<()> {
        let line = self.lines[self.current_line].trim();
        if let Some(package) = line.strip_prefix("package").and_then(|s| s.strip_suffix(';')) {
            builder.set_package(package.trim().to_string());
        }
        Ok(())
    }

    /// Parse message declarations (e.g., "message UserData { ... }")
    /// 
    /// # Arguments
    /// * `builder` - The AST builder to add the message to
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if parsing fails
    fn parse_message(&mut self, builder: &mut AstBuilder) -> Result<()> {
        let line = self.lines[self.current_line].trim();
        if let Some(name) = line.strip_prefix("message ").and_then(|s| s.split_whitespace().next()) {
            let name = name.trim_end_matches('{');
            builder.start_message(name.to_string());
            
            // Parse message body including fields and options
            self.parse_message_body(builder)?;
            builder.end_message();
        }
        Ok(())
    }

    /// Parse the body of a message declaration, including fields and options
    /// 
    /// # Arguments
    /// * `builder` - The AST builder to add message elements to
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if parsing fails
    fn parse_message_body(&mut self, builder: &mut AstBuilder) -> Result<()> {
        let mut brace_count = 1; // We already have one opening brace
        self.current_line += 1;
        
        while self.current_line < self.lines.len() && brace_count > 0 {
            let line = self.lines[self.current_line].trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with("//") {
                self.current_line += 1;
                continue;
            }
            
            // Track brace nesting
            if line.contains('{') {
                brace_count += line.matches('{').count();
            }
            if line.contains('}') {
                brace_count -= line.matches('}').count();
                if brace_count == 0 {
                    break;
                }
            }
            
            // Parse message options (like DMXP channel options)
            if line.starts_with("option") {
                self.parse_message_option(builder)?;
            }
            // Parse message fields
            else if self.is_field_line(line) {
                self.parse_field(builder)?;
            }
            
            self.current_line += 1;
        }
        Ok(())
    }

    /// Parse message-level options, particularly DMXP channel options
    /// 
    /// # Arguments
    /// * `builder` - The AST builder to add options to
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if parsing fails
    fn parse_message_option(&mut self, builder: &mut AstBuilder) -> Result<()> {
        let line = self.lines[self.current_line].trim();
        
        // Parse DMXP channel option
        if line.contains("dmxp_channel") {
            if let Some(channel_name) = self.extract_string_value(line, "dmxp_channel") {
                let dmxp_options = DmxpMessageOptions {
                    channel: Some(channel_name),
                    persistent: None,
                    buffer_size: None,
                    wal_enabled: None,
                    swap_enabled: None,
                    priority: None,
                };
                builder.set_dmxp_message_options(dmxp_options);
            }
        }
        Ok(())
    }

    /// Parse message fields (e.g., "string user_id = 1;")
    /// 
    /// # Arguments
    /// * `builder` - The AST builder to add the field to
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if parsing fails
    fn parse_field(&mut self, builder: &mut AstBuilder) -> Result<()> {
        let line = self.lines[self.current_line].trim();
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() >= 3 {
            let field_type = self.parse_field_type(parts[0]);
            let name = parts[1];
            
            // Debug output
            println!("DEBUG: Parsing field line: '{}'", line);
            println!("DEBUG: Parts: {:?}", parts);
            
            // Extract number from "name = number;" format
            let number_part = parts[2].trim_end_matches(';');
            let number = if number_part.contains('=') {
                // Handle "name = number" format
                let num_str = number_part.split('=').nth(1).unwrap_or("0").trim();
                println!("DEBUG: Number string from = format: '{}'", num_str);
                if num_str.is_empty() {
                    return Err(anyhow::anyhow!("Empty number in field: {}", line));
                }
                num_str.parse::<i32>()?
            } else {
                // Handle "number;" format
                println!("DEBUG: Number string from direct format: '{}'", number_part);
                if number_part.is_empty() {
                    return Err(anyhow::anyhow!("Empty number in field: {}", line));
                }
                number_part.parse::<i32>()?
            };
            
            let field = Field {
                name: name.to_string(),
                field_type,
                number,
                label: if line.contains("repeated") { FieldLabel::Repeated } else { FieldLabel::Optional },
                options: Vec::new(),
                default_value: None,
            };
            
            builder.add_field(field);
        }
        Ok(())
    }

    /// Parse field types from string representation to FieldType enum
    /// 
    /// # Arguments
    /// * `type_str` - The string representation of the field type
    /// 
    /// # Returns
    /// * `FieldType` - The parsed field type
    fn parse_field_type(&self, type_str: &str) -> FieldType {
        match type_str {
            "double" => FieldType::Double,
            "float" => FieldType::Float,
            "int32" => FieldType::Int32,
            "int64" => FieldType::Int64,
            "uint32" => FieldType::Uint32,
            "uint64" => FieldType::Uint64,
            "sint32" => FieldType::Sint32,
            "sint64" => FieldType::Sint64,
            "fixed32" => FieldType::Fixed32,
            "fixed64" => FieldType::Fixed64,
            "sfixed32" => FieldType::Sfixed32,
            "sfixed64" => FieldType::Sfixed64,
            "bool" => FieldType::Bool,
            "string" => FieldType::String,
            "bytes" => FieldType::Bytes,
            _ => FieldType::Message(type_str.to_string()),
        }
    }

    /// Parse service declarations (e.g., "service UserService { ... }")
    /// 
    /// # Arguments
    /// * `builder` - The AST builder to add the service to
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if parsing fails
    fn parse_service(&mut self, builder: &mut AstBuilder) -> Result<()> {
        let line = self.lines[self.current_line].trim();
        if let Some(name) = line.strip_prefix("service ").and_then(|s| s.split_whitespace().next()) {
            let name = name.trim_end_matches('{');
            builder.start_service(name.to_string());
            
            // Parse service body including methods and options
            self.parse_service_body(builder)?;
            builder.end_service();
        }
        Ok(())
    }

    /// Parse the body of a service declaration, including methods and options
    /// 
    /// # Arguments
    /// * `builder` - The AST builder to add service elements to
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if parsing fails
    fn parse_service_body(&mut self, builder: &mut AstBuilder) -> Result<()> {
        let mut brace_count = 1;
        self.current_line += 1;
        
        while self.current_line < self.lines.len() && brace_count > 0 {
            let line = self.lines[self.current_line].trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with("//") {
                self.current_line += 1;
                continue;
            }
            
            // Track brace nesting
            if line.contains('{') {
                brace_count += line.matches('{').count();
            }
            if line.contains('}') {
                brace_count -= line.matches('}').count();
                if brace_count == 0 {
                    break;
                }
            }
            
            // Parse service options (like DMXP channel options)
            if line.starts_with("option") {
                self.parse_service_option(builder)?;
            }
            // Parse RPC methods
            else if line.starts_with("rpc") {
                self.parse_method(builder)?;
            }
            
            self.current_line += 1;
        }
        Ok(())
    }

    /// Parse service-level options, particularly DMXP channel options
    /// 
    /// This function is production-ready and handles multiple DMXP channel options
    /// by collecting them and setting them as a complete list on the service.
    /// 
    /// # Arguments
    /// * `builder` - The AST builder to add options to
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if parsing fails
    fn parse_service_option(&mut self, builder: &mut AstBuilder) -> Result<()> {
        let line = self.lines[self.current_line].trim();
        
        // Parse DMXP channels option - handle multiple channel declarations
        if line.contains("dmxp_channels") {
            if let Some(channel_name) = self.extract_string_value(line, "dmxp_channels") {
                // Get existing service options or create new ones
                let mut existing_options = builder.current_service
                    .as_ref()
                    .and_then(|s| s.dmxp_options.clone())
                    .unwrap_or_else(|| DmxpServiceOptions {
                        channels: Vec::new(),
                        timeout_ms: None,
                        retry_count: None,
                    });
                
                // Add the new channel to the existing list
                existing_options.channels.push(channel_name);
                
                // Set the updated options back on the service
                builder.set_dmxp_service_options(existing_options);
            }
        }
        
        // Parse other service options (timeout, retry count, etc.)
        if line.contains("dmxp_timeout_ms") {
            if let Some(timeout_str) = self.extract_string_value(line, "dmxp_timeout_ms") {
                if let Ok(timeout_ms) = timeout_str.parse::<u32>() {
                    let mut existing_options = builder.current_service
                        .as_ref()
                        .and_then(|s| s.dmxp_options.clone())
                        .unwrap_or_else(|| DmxpServiceOptions {
                            channels: Vec::new(),
                            timeout_ms: None,
                            retry_count: None,
                        });
                    
                    existing_options.timeout_ms = Some(timeout_ms);
                    builder.set_dmxp_service_options(existing_options);
                }
            }
        }
        
        if line.contains("dmxp_retry_count") {
            if let Some(retry_str) = self.extract_string_value(line, "dmxp_retry_count") {
                if let Ok(retry_count) = retry_str.parse::<u32>() {
                    let mut existing_options = builder.current_service
                        .as_ref()
                        .and_then(|s| s.dmxp_options.clone())
                        .unwrap_or_else(|| DmxpServiceOptions {
                            channels: Vec::new(),
                            timeout_ms: None,
                            retry_count: None,
                        });
                    
                    existing_options.retry_count = Some(retry_count);
                    builder.set_dmxp_service_options(existing_options);
                }
            }
        }
        
        Ok(())
    }

    /// Parse RPC method declarations (e.g., "rpc GetUser(GetUserRequest) returns (GetUserResponse);")
    /// 
    /// # Arguments
    /// * `builder` - The AST builder to add the method to
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if parsing fails
    fn parse_method(&mut self, builder: &mut AstBuilder) -> Result<()> {
        let line = self.lines[self.current_line].trim();
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() >= 4 {
            let name = parts[1];
            let input_type = parts[2].trim_matches('(');
            let output_type = parts[4].trim_matches(')');
            
            let method = Method {
                name: name.to_string(),
                input_type: input_type.to_string(),
                output_type: output_type.to_string(),
                options: Vec::new(),
                dmxp_options: None,
            };
            
            builder.add_method(method);
        }
        Ok(())
    }

    /// Parse enum declarations (e.g., "enum OrderStatus { ... }")
    /// 
    /// # Arguments
    /// * `builder` - The AST builder to add the enum to
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if parsing fails
    fn parse_enum(&mut self, builder: &mut AstBuilder) -> Result<()> {
        let line = self.lines[self.current_line].trim();
        if let Some(name) = line.strip_prefix("enum ").and_then(|s| s.split_whitespace().next()) {
            let name = name.trim_end_matches('{');
            builder.start_enum(name.to_string());
            
            // Parse enum body including values
            self.parse_enum_body(builder)?;
            builder.end_enum();
        }
        Ok(())
    }

    /// Parse the body of an enum declaration, including enum values
    /// 
    /// # Arguments
    /// * `builder` - The AST builder to add enum elements to
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if parsing fails
    fn parse_enum_body(&mut self, builder: &mut AstBuilder) -> Result<()> {
        let mut brace_count = 1;
        self.current_line += 1;
        
        while self.current_line < self.lines.len() && brace_count > 0 {
            let line = self.lines[self.current_line].trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with("//") {
                self.current_line += 1;
                continue;
            }
            
            // Track brace nesting
            if line.contains('{') {
                brace_count += line.matches('{').count();
            }
            if line.contains('}') {
                brace_count -= line.matches('}').count();
                if brace_count == 0 {
                    break;
                }
            }
            
            // Parse enum values
            if line.contains('=') && !line.starts_with("option") {
                self.parse_enum_value(builder)?;
            }
            
            self.current_line += 1;
        }
        Ok(())
    }

    /// Parse enum value declarations (e.g., "ORDER_STATUS_PENDING = 1;")
    /// 
    /// # Arguments
    /// * `builder` - The AST builder to add the enum value to
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error if parsing fails
    fn parse_enum_value(&mut self, builder: &mut AstBuilder) -> Result<()> {
        let line = self.lines[self.current_line].trim();
        let parts: Vec<&str> = line.split('=').collect();
        
        if parts.len() == 2 {
            let name = parts[0].trim();
            let number = parts[1].trim_end_matches(';').trim().parse::<i32>()?;
            
            let enum_value = EnumValue {
                name: name.to_string(),
                number,
                options: Vec::new(),
            };
            
            builder.add_enum_value(enum_value);
        }
        Ok(())
    }

    /// Check if a line represents a field declaration
    /// 
    /// Uses heuristics to determine if a line contains a field definition
    /// by checking for type, name, and number pattern.
    /// 
    /// # Arguments
    /// * `line` - The line to check
    /// 
    /// # Returns
    /// * `bool` - True if the line appears to be a field declaration
    fn is_field_line(&self, line: &str) -> bool {
        // Simple heuristic: field lines contain a type, name, and number
        let parts: Vec<&str> = line.split_whitespace().collect();
        parts.len() >= 3 && 
        !line.starts_with("message") && 
        !line.starts_with("service") && 
        !line.starts_with("enum") &&
        !line.starts_with("option") &&
        !line.starts_with("rpc") &&
        (parts[2].contains('=') || (parts.len() > 3 && parts[3].contains('=')))
    }

    /// Extract string values from option declarations
    /// 
    /// Parses option lines to extract quoted string values for specific keys.
    /// 
    /// # Arguments
    /// * `line` - The line containing the option
    /// * `key` - The key to extract the value for
    /// 
    /// # Returns
    /// * `Option<String>` - The extracted string value or None if not found
    fn extract_string_value(&self, line: &str, key: &str) -> Option<String> {
        if let Some(start) = line.find(&format!("{} = ", key)) {
            let value_start = start + key.len() + 3;
            let value_part = &line[value_start..];
            if let Some(end) = value_part.find(';') {
                let value = &value_part[..end];
                return value.trim_matches('"').to_string().into();
            }
        }
        None
    }
}

