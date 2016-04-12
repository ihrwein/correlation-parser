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

enum MockTime {
    Format(Box<Fn(&str) -> MockTemplate>),
    Compile(Box<Fn(&str) -> Result<MockTemplate, CompileError>>)
}

pub struct MockTemplateFactory (MockTime);

impl MockTemplateFactory {
    // returns the value which is compiled as an error
    pub fn compile_error() -> MockTemplateFactory {
        MockTemplateFactory(MockTime::Compile(Box::new(move |value| { Err(CompileError(value.to_owned())) })))
    }
    // return a literal from format()
    pub fn format_literal(value: &'static str) -> MockTemplateFactory {
        MockTemplateFactory(MockTime::Format(Box::new(move |_| {
            let template = MockTemplate {
                with_context: Box::new(move |_, _| { value }),
            };
            template
        })))
    }
    // returns the special context_id value from format
    pub fn format_context_id() -> MockTemplateFactory {
        MockTemplateFactory::format_literal(CONTEXT_ID)
    }
}

impl TemplateFactory<Message> for MockTemplateFactory {
    type Template = MockTemplate;
    fn compile(&self, value: &str) -> Result<MockTemplate, CompileError> {
        match self.0 {
            MockTime::Format(ref clojure) => Ok(clojure(value)),
            MockTime::Compile(ref clojure) => clojure(value)
        }
    }
}

#[test]
fn test_mock_template_factory_can_generate_errors() {
    let factory = MockTemplateFactory::compile_error();
    let expected = CompileError("ERROR".to_owned());
    let actual = factory.compile("ERROR").err().unwrap();
    assert_eq!(expected, actual);
}
