use serde::{Deserialize, Serialize};

/// Root AST node representing the entire protobuf file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtoFile {
    pub syntax: String,
    pub package: String,
    pub options: Vec<ProtoOption>,
    pub messages: Vec<Message>,
    pub services: Vec<Service>,
    pub enums: Vec<Enum>,
    pub dmxp_channels: Vec<DmxpChannel>,
}


/// Option definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtoOption {
    pub name: String,
    pub value: OptionValue,
}

/// Option value types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptionValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Identifier(String),
}

/// Message definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub name: String,
    pub fields: Vec<Field>,
    pub nested_messages: Vec<Message>,
    pub nested_enums: Vec<Enum>,
    pub options: Vec<ProtoOption>,
    pub dmxp_options: Option<DmxpMessageOptions>,
}

/// Field definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
    pub number: i32,
    pub label: FieldLabel,
    pub options: Vec<ProtoOption>,
    pub default_value: Option<OptionValue>,
}

/// Field type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    // Scalar types
    Double,
    Float,
    Int32,
    Int64,
    Uint32,
    Uint64,
    Sint32,
    Sint64,
    Fixed32,
    Fixed64,
    Sfixed32,
    Sfixed64,
    Bool,
    String,
    Bytes,
    // User-defined types
    Message(String),
    Enum(String),
    // Map type
    Map(Box<FieldType>, Box<FieldType>),
}

/// Field label (repeated, optional, required)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldLabel {
    Optional,
    Required,
    Repeated,
}

/// Service definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub methods: Vec<Method>,
    pub options: Vec<ProtoOption>,
    pub dmxp_options: Option<DmxpServiceOptions>,
}

/// Service method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    pub input_type: String,
    pub output_type: String,
    pub options: Vec<ProtoOption>,
    pub dmxp_options: Option<DmxpMethodOptions>,
}

/// Enum definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub values: Vec<EnumValue>,
    pub options: Vec<ProtoOption>,
}

/// Enum value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumValue {
    pub name: String,
    pub number: i32,
    pub options: Vec<ProtoOption>,
}

/// Extension definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    pub name: String,
    pub field_type: FieldType,
    pub number: i32,
    pub options: Vec<ProtoOption>,
}

/// DMXP-specific message options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DmxpMessageOptions {
    pub channel: Option<String>,
    pub persistent: Option<bool>,
    pub buffer_size: Option<u32>,
    pub wal_enabled: Option<bool>,
    pub swap_enabled: Option<bool>,
    pub priority: Option<u32>,
}

/// DMXP-specific service options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DmxpServiceOptions {
    pub channels: Vec<String>,
    pub timeout_ms: Option<u32>,
    pub retry_count: Option<u32>,
}

/// DMXP-specific method options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DmxpMethodOptions {
    pub channel: Option<String>,
    pub timeout_ms: Option<u32>,
    pub is_async: Option<bool>,
}

/// DMXP channel definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DmxpChannel {
    pub name: String,
    pub message_type: String,
    pub direction: ChannelDirection,
    pub options: DmxpChannelOptions,
}

/// Channel direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChannelDirection {
    Publish,
    Subscribe,
    Bidirectional,
}

/// DMXP channel options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DmxpChannelOptions {
    pub buffer_size: Option<u32>,
    pub persistent: Option<bool>,
    pub wal_enabled: Option<bool>,
    pub swap_enabled: Option<bool>,
    pub priority: Option<u32>,
    pub timeout_ms: Option<u32>,
}

/// AST Builder for constructing the parse tree
pub struct AstBuilder {
    pub current_file: ProtoFile,
    pub current_message: Option<Message>,
    pub current_service: Option<Service>,
    pub current_enum: Option<Enum>,
    pub message_stack: Vec<Message>,
}