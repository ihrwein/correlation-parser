use Template;
use Event;
use TemplateFactory;
use Message;
use CompileError;

use std::sync::Arc;

pub struct MockTemplate(pub String);

impl Template for MockTemplate {
    type Event = Message;
    fn format_with_context(&self, messages: &[Arc<Self::Event>], context_id: &str) -> &str {
        &self.0
    }
    fn format(&self, message: &Self::Event) -> &str {
        &self.0
    }
}

pub struct MockTemplateFactory(Box<Fn() -> Result<MockTemplate, CompileError>>);

impl MockTemplateFactory {
    pub fn error(error: &'static str) -> MockTemplateFactory {
        MockTemplateFactory(Box::new(move || { Err(CompileError(error.to_owned())) }))
    }
    pub fn static_template(value: &'static str) -> MockTemplateFactory {
        MockTemplateFactory(Box::new(move || {
            Ok(MockTemplate(value.to_owned()))
        }))
    }
}

impl TemplateFactory<Message> for MockTemplateFactory {
    type Template = MockTemplate;
    fn compile(&self, value: &str) -> Result<MockTemplate, CompileError> {
        self.0()
    }
}
