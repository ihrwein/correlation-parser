// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use TemplatableString;
use super::MessageAction;
use super::InjectMode;
use config::action::ExecCondition;
use Event;

use std::collections::BTreeMap;

pub struct MessageActionBuilder<E> where E: Event {
    uuid: String,
    name: Option<String>,
    message: TemplatableString<E>,
    values: BTreeMap<String, TemplatableString<E>>,
    when: ExecCondition,
    inject_mode: InjectMode,
}

impl<E> MessageActionBuilder<E> where E: Event {
    pub fn new<S: Into<String>>(uuid: S, message: S) -> MessageActionBuilder<E> {
        MessageActionBuilder {
            uuid: uuid.into(),
            name: None,
            message: TemplatableString::Literal(message.into()),
            values: BTreeMap::default(),
            when: ExecCondition::default(),
            inject_mode: InjectMode::default(),
        }
    }

    pub fn name<S: Into<String>>(mut self, name: Option<S>) -> MessageActionBuilder<E> {
        self.name = name.map(|name| name.into());
        self
    }

    pub fn when(mut self, when: ExecCondition) -> MessageActionBuilder<E> {
        self.when = when;
        self
    }

    pub fn values(mut self, values: BTreeMap<String, TemplatableString<E>>) -> MessageActionBuilder<E> {
        self.values = values;
        self
    }

    pub fn pair<S: Into<String>>(mut self, key: S, value: S) -> MessageActionBuilder<E> {
        self.values.insert(key.into(), TemplatableString::Literal(value.into()));
        self
    }

    pub fn inject_mode(mut self, mode: InjectMode) -> MessageActionBuilder<E> {
        self.inject_mode = mode;
        self
    }

    pub fn build(self) -> MessageAction<E> {
        MessageAction {
            uuid: self.uuid,
            name: self.name,
            message: self.message,
            values: self.values,
            when: self.when,
            inject_mode: self.inject_mode,
        }
    }
}
