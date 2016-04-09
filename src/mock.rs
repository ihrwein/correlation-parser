use syslog_ng_common::LogMessage;
use correlation::{Message, Event, EventIds};

use std::borrow::Borrow;

#[derive(Clone)]
pub struct MockEvent(pub Message);

impl Event for MockEvent {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key)
    }
    fn uuid(&self) -> &str {
        self.0.uuid()
    }
    fn ids(&self) -> EventIds {
        self.0.ids()
    }
    fn new(uuid: &str, message: &str) -> Self {
        MockEvent(Message::new(uuid, message))
    }
    fn set_name(&mut self, name: Option<&str>) {
        self.0.set_name(name);
    }
    fn name(&self) -> Option<&str> {
        self.0.name()
    }
    fn set(&mut self, key: &str, value: &str) {
        self.0.set(key, value);
    }
    fn set_message(&mut self, message: &str) {
        self.0.set_message(message);
    }
    fn message(&self) -> &str {
        self.0.message()
    }
}

impl Into<LogMessage> for MockEvent {
    fn into(self) -> LogMessage {
        let mut logmsg = LogMessage::new();
        for (k, v) in self.0.values.iter() {
            logmsg.insert(k.borrow(), v.borrow());
        }
        logmsg.insert("MESSAGE", &self.0.message);
        logmsg
    }
}
