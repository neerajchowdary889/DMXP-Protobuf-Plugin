use crate::ast::structs::*;


impl AstBuilder {
    pub fn new() -> Self {
        Self {
            current_file: ProtoFile {
                syntax: "proto3".to_string(),
                package: String::new(),
                options: Vec::new(),
                messages: Vec::new(),
                services: Vec::new(),
                enums: Vec::new(),
                extensions: Vec::new(),
                dmxp_channels: Vec::new(),
            },
            current_message: None,
            current_service: None,
            current_enum: None,
        }
    }

    pub fn set_syntax(&mut self, syntax: String) {
        self.current_file.syntax = syntax;
    }

    pub fn set_package(&mut self, package: String) {
        self.current_file.package = package;
    }

    pub fn add_option(&mut self, option: ProtoOption) {
        self.current_file.options.push(option);
    }

    pub fn start_message(&mut self, name: String) {
        if let Some(current_msg) = self.current_message.take() {
            if let Some(parent_msg) = self.current_message.as_mut() {
                parent_msg.nested_messages.push(current_msg);
            } else {
                self.current_file.messages.push(current_msg);
            }
        }
        
        self.current_message = Some(Message {
            name,
            fields: Vec::new(),
            nested_messages: Vec::new(),
            nested_enums: Vec::new(),
            options: Vec::new(),
            dmxp_options: None,
        });
    }

    pub fn end_message(&mut self) {
        if let Some(message) = self.current_message.take() {
            if let Some(parent_msg) = self.current_message.as_mut() {
                parent_msg.nested_messages.push(message);
            } else {
                self.current_file.messages.push(message);
            }
        }
    }

    pub fn add_field(&mut self, field: Field) {
        if let Some(current_msg) = self.current_message.as_mut() {
            current_msg.fields.push(field);
        }
    }

    pub fn add_message_option(&mut self, option: ProtoOption) {
        if let Some(current_msg) = self.current_message.as_mut() {
            current_msg.options.push(option);
        }
    }

    pub fn set_dmxp_message_options(&mut self, options: DmxpMessageOptions) {
        if let Some(current_msg) = self.current_message.as_mut() {
            current_msg.dmxp_options = Some(options);
        }
    }

    pub fn get_dmxp_message_options(&mut self) -> Option<&mut DmxpMessageOptions> {
        self.current_message.as_mut().and_then(|m| m.dmxp_options.as_mut())
    }

    pub fn start_service(&mut self, name: String) {
        self.current_service = Some(Service {
            name,
            methods: Vec::new(),
            options: Vec::new(),
            dmxp_options: None,
        });
    }

    pub fn end_service(&mut self) {
        if let Some(service) = self.current_service.take() {
            self.current_file.services.push(service);
        }
    }

    pub fn add_method(&mut self, method: Method) {
        if let Some(current_svc) = self.current_service.as_mut() {
            current_svc.methods.push(method);
        }
    }

    pub fn add_service_option(&mut self, option: ProtoOption) {
        if let Some(current_svc) = self.current_service.as_mut() {
            current_svc.options.push(option);
        }
    }

    pub fn set_dmxp_service_options(&mut self, options: DmxpServiceOptions) {
        if let Some(current_svc) = self.current_service.as_mut() {
            current_svc.dmxp_options = Some(options);
        }
    }

    pub fn start_enum(&mut self, name: String) {
        if let Some(current_enum) = self.current_enum.take() {
            if let Some(current_msg) = self.current_message.as_mut() {
                current_msg.nested_enums.push(current_enum);
            } else {
                self.current_file.enums.push(current_enum);
            }
        }
        
        self.current_enum = Some(Enum {
            name,
            values: Vec::new(),
            options: Vec::new(),
        });
    }

    pub fn end_enum(&mut self) {
        if let Some(enum_def) = self.current_enum.take() {
            if let Some(current_msg) = self.current_message.as_mut() {
                current_msg.nested_enums.push(enum_def);
            } else {
                self.current_file.enums.push(enum_def);
            }
        }
    }

    pub fn add_enum_value(&mut self, value: EnumValue) {
        if let Some(current_enum) = self.current_enum.as_mut() {
            current_enum.values.push(value);
        }
    }

    pub fn add_extension(&mut self, extension: Extension) {
        self.current_file.extensions.push(extension);
    }

    pub fn add_dmxp_channel(&mut self, channel: DmxpChannel) {
        self.current_file.dmxp_channels.push(channel);
    }

    pub fn build(self) -> ProtoFile {
        self.current_file
    }
}

impl Default for AstBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for working with the AST
impl ProtoFile {
    /// Find a message by name
    pub fn find_message(&self, name: &str) -> Option<&Message> {
        self.messages.iter().find(|m| m.name == name)
    }

    /// Find a service by name
    pub fn find_service(&self, name: &str) -> Option<&Service> {
        self.services.iter().find(|s| s.name == name)
    }

    /// Find an enum by name
    pub fn find_enum(&self, name: &str) -> Option<&Enum> {
        self.enums.iter().find(|e| e.name == name)
    }

    /// Get all DMXP channels
    pub fn get_dmxp_channels(&self) -> &Vec<DmxpChannel> {
        &self.dmxp_channels
    }

    /// Get messages with DMXP options
    pub fn get_dmxp_messages(&self) -> Vec<&Message> {
        self.messages
            .iter()
            .filter(|m| m.dmxp_options.is_some())
            .collect()
    }

    /// Get services with DMXP options
    pub fn get_dmxp_services(&self) -> Vec<&Service> {
        self.services
            .iter()
            .filter(|s| s.dmxp_options.is_some())
            .collect()
    }
}

impl Message {
    /// Check if message has DMXP channel
    pub fn has_dmxp_channel(&self) -> bool {
        self.dmxp_options
            .as_ref()
            .map(|opts| opts.channel.is_some())
            .unwrap_or(false)
    }

    /// Get DMXP channel name
    pub fn get_dmxp_channel(&self) -> Option<&String> {
        self.dmxp_options
            .as_ref()
            .and_then(|opts| opts.channel.as_ref())
    }
}

impl Service {
    /// Check if service has DMXP channels
    pub fn has_dmxp_channels(&self) -> bool {
        self.dmxp_options
            .as_ref()
            .map(|opts| !opts.channels.is_empty())
            .unwrap_or(false)
    }

    /// Get DMXP channel names
    pub fn get_dmxp_channels(&self) -> Vec<&String> {
        self.dmxp_options
            .as_ref()
            .map(|opts| opts.channels.iter().collect())
            .unwrap_or_default()
    }
}
