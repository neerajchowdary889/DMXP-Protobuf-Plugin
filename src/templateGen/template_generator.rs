use crate::ast::*;
use anyhow::Result;
use std::collections::HashMap;

/// Supported target languages for code generation
#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    Rust,
    Go,
}

/// Main template generator that coordinates code generation for different languages
pub struct TemplateGenerator {
    language: Language,
    options: GeneratorOptions,
}

/// Options for code generation
#[derive(Debug, Clone)]
pub struct GeneratorOptions {
    /// Whether to generate DMXP-specific code
    pub include_dmxp: bool,
    /// Whether to generate async/await patterns
    pub use_async: bool,
    /// Custom package/module name override
    pub package_override: Option<String>,
    /// Additional imports to include
    pub extra_imports: Vec<String>,
}

impl Default for GeneratorOptions {
    fn default() -> Self {
        Self {
            include_dmxp: true,
            use_async: true,
            package_override: None,
            extra_imports: Vec::new(),
        }
    }
}

impl TemplateGenerator {
    /// Create a new template generator for the specified language
    pub fn new(language: Language) -> Self {
        Self {
            language,
            options: GeneratorOptions::default(),
        }
    }

    /// Create a new template generator with custom options
    pub fn new_with_options(language: Language, options: GeneratorOptions) -> Self {
        Self { language, options }
    }

    /// Generate code from the AST
    pub fn generate(&self, proto_file: &ProtoFile) -> Result<String> {
        match self.language {
            Language::Rust => self.generate_rust(proto_file),
            Language::Go => self.generate_go(proto_file),
        }
    }

    /// Generate Rust code
    fn generate_rust(&self, proto_file: &ProtoFile) -> Result<String> {
        use crate::templateGen::rust_generator::RustGenerator;
        let generator = RustGenerator::new(self.options.clone());
        generator.generate(proto_file)
    }

    /// Generate Go code
    fn generate_go(&self, proto_file: &ProtoFile) -> Result<String> {
        use crate::templateGen::go_generator::GoGenerator;
        let generator = GoGenerator::new(self.options.clone());
        generator.generate(proto_file)
    }

    /// Set generator options
    pub fn with_options(mut self, options: GeneratorOptions) -> Self {
        self.options = options;
        self
    }

    /// Set the target language
    pub fn with_language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }
}

/// Trait for language-specific code generators
pub trait CodeGenerator {
    /// Generate code from the AST
    fn generate(&self, proto_file: &ProtoFile) -> Result<String>;
}

/// Helper functions for common code generation patterns
pub mod helpers {
    use super::*;

    /// Convert protobuf field type to language-specific type
    pub fn convert_field_type(field_type: &FieldType, language: &Language) -> String {
        match language {
            Language::Rust => convert_to_rust_type(field_type),
            Language::Go => convert_to_go_type(field_type),
        }
    }

    /// Convert to Rust type
    fn convert_to_rust_type(field_type: &FieldType) -> String {
        match field_type {
            FieldType::Double => "f64".to_string(),
            FieldType::Float => "f32".to_string(),
            FieldType::Int32 => "i32".to_string(),
            FieldType::Int64 => "i64".to_string(),
            FieldType::Uint32 => "u32".to_string(),
            FieldType::Uint64 => "u64".to_string(),
            FieldType::Sint32 => "i32".to_string(),
            FieldType::Sint64 => "i64".to_string(),
            FieldType::Fixed32 => "u32".to_string(),
            FieldType::Fixed64 => "u64".to_string(),
            FieldType::Sfixed32 => "i32".to_string(),
            FieldType::Sfixed64 => "i64".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::String => "String".to_string(),
            FieldType::Bytes => "Vec<u8>".to_string(),
            FieldType::Message(name) => name.clone(),
            FieldType::Enum(name) => name.clone(),
            FieldType::Map(key_type, value_type) => {
                format!("HashMap<{}, {}>", 
                    convert_to_rust_type(key_type), 
                    convert_to_rust_type(value_type))
            }
        }
    }

    /// Convert to Go type
    fn convert_to_go_type(field_type: &FieldType) -> String {
        match field_type {
            FieldType::Double => "float64".to_string(),
            FieldType::Float => "float32".to_string(),
            FieldType::Int32 => "int32".to_string(),
            FieldType::Int64 => "int64".to_string(),
            FieldType::Uint32 => "uint32".to_string(),
            FieldType::Uint64 => "uint64".to_string(),
            FieldType::Sint32 => "int32".to_string(),
            FieldType::Sint64 => "int64".to_string(),
            FieldType::Fixed32 => "uint32".to_string(),
            FieldType::Fixed64 => "uint64".to_string(),
            FieldType::Sfixed32 => "int32".to_string(),
            FieldType::Sfixed64 => "int64".to_string(),
            FieldType::Bool => "bool".to_string(),
            FieldType::String => "string".to_string(),
            FieldType::Bytes => "[]byte".to_string(),
            FieldType::Message(name) => format!("*{}", name),
            FieldType::Enum(name) => name.clone(),
            FieldType::Map(key_type, value_type) => {
                format!("map[{}]{}", 
                    convert_to_go_type(key_type), 
                    convert_to_go_type(value_type))
            }
        }
    }



    /// Convert field label to language-specific representation
    pub fn convert_field_label(label: &FieldLabel, language: &Language) -> String {
        match language {
            Language::Rust => match label {
                FieldLabel::Optional => "Option<".to_string(),
                FieldLabel::Required => "".to_string(),
                FieldLabel::Repeated => "Vec<".to_string(),
            },
            Language::Go => match label {
                FieldLabel::Optional => "*".to_string(),
                FieldLabel::Required => "".to_string(),
                FieldLabel::Repeated => "[]".to_string(),
            },

        }
    }

    /// Generate field name in language-specific style
    pub fn convert_field_name(name: &str, language: &Language) -> String {
        match language {
            Language::Rust => name.to_string(), // snake_case
            Language::Go => to_pascal_case(name), // PascalCase

        }
    }

    /// Convert string to PascalCase
    fn to_pascal_case(s: &str) -> String {
        s.split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().chain(chars).collect(),
                }
            })
            .collect()
    }

    /// Generate DMXP channel code for a message
    pub fn generate_dmxp_channel_code(message: &Message, language: &Language) -> String {
        if let Some(dmxp_opts) = &message.dmxp_options {
            if let Some(channel) = &dmxp_opts.channel {
                match language {
                    Language::Rust => generate_rust_dmxp_code(message, channel),
                    Language::Go => generate_go_dmxp_code(message, channel),

                }
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }

    fn generate_rust_dmxp_code(message: &Message, channel: &str) -> String {
        format!(
            r#"
impl {} {{
    pub fn publish(&self, publisher: &dmxp::Publisher) -> Result<(), dmxp::Error> {{
        publisher.publish("{}", self)
    }}
    
    pub fn subscribe(callback: impl Fn({}) -> Result<(), dmxp::Error> + Send + Sync + 'static) -> Result<(), dmxp::Error> {{
        dmxp::subscribe("{}", callback)
    }}
}}"#,
            message.name, channel, message.name, channel
        )
    }

    fn generate_go_dmxp_code(message: &Message, channel: &str) -> String {
        format!(
            r#"
func (m *{}) Publish(publisher *dmxp.Publisher) error {{
    return publisher.Publish("{}", m)
}}

func Subscribe{}(callback func(*{}) error) error {{
    return dmxp.Subscribe("{}", callback)
}}"#,
            message.name, channel, message.name, message.name, channel
        )
    }

}
