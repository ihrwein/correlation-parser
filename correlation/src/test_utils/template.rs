use Template;
use Event;
use TemplateFactory;
use Message;
use CompileError;

use std::sync::Arc;

pub const CONTEXT_ID: &'static str = "${context_id}";

pub struct MockTemplate {
    pub with_context: Box<for<'a> Fn(&[Arc<Message>], &str) -> &'a str + Send>,
}

impl Template for MockTemplate {
    type Event = Message;
    fn format_with_context(&self, messages: &[Arc<Self::Event>], context_id: &str) -> &str {
        (self.with_context)(messages, context_id)
    }
}

pub struct MockTemplateFactory(Box<Fn() -> Result<MockTemplate, CompileError>>);

impl MockTemplateFactory {
    pub fn compile_error(error: &'static str) -> MockTemplateFactory {
        MockTemplateFactory(Box::new(move || { Err(CompileError(error.to_owned())) }))
    }
    pub fn literal(value: &'static str) -> MockTemplateFactory {
        MockTemplateFactory(Box::new(move || {
            let template = MockTemplate {
                with_context: Box::new(move |_, _| { value }),
            };
            Ok(template)
        }))
    }
    pub fn context_id() -> MockTemplateFactory {
        MockTemplateFactory::literal(CONTEXT_ID)
    }
}

impl TemplateFactory<Message> for MockTemplateFactory {
    type Template = MockTemplate;
    fn compile(&self, _: &str) -> Result<MockTemplate, CompileError> {
        self.0()
    }
}

#[test]
fn test_mock_template_factory_can_generate_errors() {
    let factory = MockTemplateFactory::compile_error("ERROR");
    let expected = Err(CompileError("ERROR".to_owned()));
    let actual = factory.compile("doesn't matter");
    assert_eq!(expected, actual);
}
