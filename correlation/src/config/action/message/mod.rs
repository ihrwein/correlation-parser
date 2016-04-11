// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use action::Action;
use context::base::BaseContext;
use dispatcher::Response;
use dispatcher::response::ResponseSender;
use Event;
use TemplatableString;
use Template;

use std::collections::BTreeMap;
use std::borrow::Borrow;
use state::State;
use super::ExecCondition;

pub use self::builder::MessageActionBuilder;

mod deser;
mod builder;
#[cfg(test)]
mod test;

pub const CONTEXT_UUID: &'static str = "context_uuid";
pub const CONTEXT_NAME: &'static str = "context_name";
pub const CONTEXT_LEN: &'static str = "context_len";
pub const MESSAGES: &'static str = "messages";

pub struct MessageAction<T> {
    pub uuid: String,
    pub name: Option<String>,
    pub message: T,
    pub values: BTreeMap<String, T>,
    pub when: ExecCondition,
    pub inject_mode: InjectMode,
}

impl<T> MessageAction<T> {
    pub fn uuid(&self) -> &String {
        &self.uuid
    }
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    pub fn message(&self) -> &T {
        &self.message
    }
    pub fn values(&self) -> &BTreeMap<String, T> {
        &self.values
    }
    pub fn inject_mode(&self) -> &InjectMode {
        &self.inject_mode
    }

    fn execute<E>(&self, _state: &State<E>, _context: &BaseContext<E, T>, responder: &mut ResponseSender<E>) where E: Event, T: Template<Event=E> {
        let context_id = _context.uuid.to_hyphenated_string();
        let message = self.message.format_with_context(_state.messages(), &context_id);
        let mut event = E::new(&self.uuid, &message);
        event.set_name(self.name.as_ref().map(|name| name.borrow()));
        for (k, v) in &self.values {
            let value = v.format_with_context(_state.messages(), &context_id);
            event.set(k, value);
        }
        let response = Alert {
            message: event,
            inject_mode: self.inject_mode.clone(),
        };
        responder.send_response(Response::Alert(response));
    }
}

impl<T> From<MessageAction<T>> for super::ActionType<T> {
    fn from(action: MessageAction<T>) -> super::ActionType<T> {
        super::ActionType::Message(action)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum InjectMode {
    Log,
    Forward,
    Loopback,
}

impl Default for InjectMode {
    fn default() -> InjectMode {
        InjectMode::Log
    }
}

#[derive(Debug, Clone)]
pub struct Alert<E: Event> {
    pub message: E,
    pub inject_mode: InjectMode,
}

impl<E, T> Action<E, T> for MessageAction<T> where E: Event, T: Template<Event=E> {
    fn on_opened(&self, state: &State<E>, context: &BaseContext<E, T>, responder: &mut ResponseSender<E>) {
        if self.when.on_opened {
            trace!("MessageAction: on_opened()");
            self.execute(state, context, responder);
        }
    }

    fn on_closed(&self, state: &State<E>, context: &BaseContext<E, T>, responder: &mut ResponseSender<E>) {
        if self.when.on_closed {
            trace!("MessageAction: on_closed()");
            self.execute(state, context, responder);
        }
    }
}
