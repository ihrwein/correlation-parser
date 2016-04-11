// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use uuid::Uuid;

use config::action::ActionType;
use conditions::Conditions;
use Event;

mod deser;
pub mod action;

pub struct ContextConfig<E> where E: Event {
    pub name: Option<String>,
    pub uuid: Uuid,
    pub conditions: Conditions,
    pub context_id: Option<Vec<String>>,
    pub actions: Vec<ActionType<E>>,
    pub patterns: Vec<String>
}

pub struct ContextConfigBuilder<E> where E: Event {
    name: Option<String>,
    uuid: Uuid,
    conditions: Conditions,
    context_id: Option<Vec<String>>,
    actions: Vec<ActionType<E>>,
    patterns: Vec<String>
}

impl<E> ContextConfigBuilder<E> where E: Event {
    pub fn new(uuid: Uuid, conditions: Conditions) -> ContextConfigBuilder<E> {
        ContextConfigBuilder {
            name: None,
            uuid: uuid,
            conditions: conditions,
            context_id: None,
            actions: Vec::new(),
            patterns: Vec::new()
        }
    }

    pub fn context_id(mut self, context_id: Option<Vec<String>>) -> ContextConfigBuilder<E> {
        self.context_id = context_id;
        self
    }

    pub fn actions(mut self, actions: Vec<ActionType<E>>) -> ContextConfigBuilder<E> {
        self.actions = actions;
        self
    }

    pub fn name(mut self, name: String) -> ContextConfigBuilder<E> {
        self.name = Some(name);
        self
    }

    pub fn patterns(mut self, patterns: Vec<String>) -> ContextConfigBuilder<E> {
        self.patterns = patterns;
        self
    }

    pub fn build(self) -> ContextConfig<E> {
        ContextConfig {
            name: self.name,
            uuid: self.uuid,
            conditions: self.conditions,
            context_id: self.context_id,
            actions: self.actions,
            patterns: self.patterns
        }
    }
}
