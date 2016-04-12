// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use super::MessageAction;
use super::InjectMode;
use config::action::ExecCondition;

use serde::de::{Deserialize, Deserializer, Error, MapVisitor, Visitor};
use std::collections::BTreeMap;
use std::marker::PhantomData;

impl<T> Deserialize for MessageAction<T> where T: Deserialize {
    fn deserialize<D>(deserializer: &mut D) -> Result<MessageAction<T>, D::Error>
        where D: Deserializer
    {
        deserializer.deserialize_struct("MessageAction", &[], MessageActionVisitor(PhantomData))
    }
}

enum Field {
    Uuid,
    Name,
    Message,
    Values,
    When,
    InjectMode,
}

impl Deserialize for Field {
    fn deserialize<D>(deserializer: &mut D) -> Result<Field, D::Error>
        where D: Deserializer
    {
        struct FieldVisitor;

        impl Visitor for FieldVisitor {
            type Value = Field;

            fn visit_str<E>(&mut self, value: &str) -> Result<Field, E>
                where E: Error
            {
                match value {
                    "name" => Ok(Field::Name),
                    "uuid" => Ok(Field::Uuid),
                    "values" => Ok(Field::Values),
                    "message" => Ok(Field::Message),
                    "when" => Ok(Field::When),
                    "inject_mode" => Ok(Field::InjectMode),
                    _ => Err(Error::custom(format!("Unexpected field: {}", value))),
                }
            }
        }

        deserializer.deserialize(FieldVisitor)
    }
}

struct MessageActionVisitor<T>(PhantomData<T>);

impl<T> Visitor for MessageActionVisitor<T> where T: Deserialize {
    type Value = MessageAction<T>;

    fn visit_map<V>(&mut self, mut visitor: V) -> Result<MessageAction<T>, V::Error>
        where V: MapVisitor
    {
        let mut name: Option<String> = None;
        let mut uuid: Option<String> = None;
        let mut message: Option<T> = None;
        let mut values: Option<BTreeMap<String, T>> = None;
        let mut when: ExecCondition = ExecCondition::new();
        let mut inject_mode = Default::default();

        while let Some(field) = try!(visitor.visit_key()) {
            match field {
                Field::Name => name = Some(try!(visitor.visit_value())),
                Field::Uuid => uuid = Some(try!(visitor.visit_value())),
                Field::Message => message = Some(try!(visitor.visit_value())),
                Field::Values => values = Some(try!(visitor.visit_value())),
                Field::When => when = try!(visitor.visit_value()),
                Field::InjectMode => inject_mode = try!(visitor.visit_value()),
            }
        }

        let uuid = match uuid {
            Some(uuid) => uuid,
            None => return visitor.missing_field("uuid"),
        };

        let message = match message {
            Some(message) => message,
            None => {
                error!("Missing 'message' field: uuid={}", &uuid);
                return visitor.missing_field("message");
            }
        };

        try!(visitor.end());

        Ok(MessageAction {
            uuid: uuid,
            message: message,
            name: name,
            values: values.unwrap_or_default(),
            when: when,
            inject_mode: inject_mode,
        })
    }
}

impl Deserialize for InjectMode {
    fn deserialize<D>(deserializer: &mut D) -> Result<InjectMode, D::Error>
        where D: Deserializer
    {
        struct FieldVisitor;

        impl Visitor for FieldVisitor {
            type Value = InjectMode;

            fn visit_str<E>(&mut self, value: &str) -> Result<InjectMode, E>
                where E: Error
            {
                match value {
                    "log" => Ok(InjectMode::Log),
                    "loopback" => Ok(InjectMode::Loopback),
                    "forward" => Ok(InjectMode::Forward),
                    _ => Err(E::custom(format!("Unexpected field: {}", value))),
                }
            }
        }

        deserializer.deserialize(FieldVisitor)
    }
}

#[cfg(test)]
mod test {
    use config::action::message::{MessageActionBuilder, MessageAction, InjectMode};
    use std::fmt::Debug;

    use serde_json::from_str;

    fn assert_message_action_eq<T: Eq + Debug>(expected: &MessageAction<T>, actual: &MessageAction<T>) {
        assert_eq!(expected.uuid(), actual.uuid());
        assert_eq!(expected.name(), actual.name());
        assert_eq!(expected.message(), actual.message());
    }

    #[test]
    fn test_given_message_as_a_json_string_when_it_is_deserialized_then_we_get_the_expected_message
        () {
        let text = r#"
        {
          "uuid": "UUID",
          "name": "NAME",
          "message": "message",
          "values": {
            "key1": "value1",
            "key2": "value2"
          }
        }
        "#;

        let expected_message = MessageActionBuilder::new("UUID", "message")
                                   .name(Some("NAME"))
                                   .pair("key1", "value1")
                                   .pair("key2", "value2")
                                   .build();
        let result = from_str::<MessageAction<String>>(text);
        let message = result.expect("Failed to deserialize a valid MessageAction object");
        assert_message_action_eq(&expected_message, &message);
    }

    #[test]
    fn test_given_message_as_a_json_string_when_only_the_required_fields_are_present_then_we_can_deserialize_the_message
        () {
        let text = r#"
        {
          "uuid": "UUID",
          "message": "message"
        }
        "#;

        let expected_message = MessageActionBuilder::new("UUID", "message").build();
        let result = from_str::<MessageAction<String>>(text);
        let message = result.expect("Failed to deserialize a valid MessageAction object");
        assert_message_action_eq(&expected_message, &message);
    }

    #[test]
    fn test_given_message_as_a_json_string_when_one_of_the_required_fields_are_not_present_then_we_get_error
        () {
        let text = r#"
        {
        }
        "#;

        let result = from_str::<MessageAction<String>>(text);
        let _ = result.err().expect("Successfully deserialized an invalid MessageAction object");
    }

    #[test]
    fn test_given_inject_modes_when_they_are_deserialized_then_we_get_the_right_result() {
        let text = r#"
        ["forward", "log", "loopback", "log"]
        "#;
        let expected = vec![InjectMode::Forward,
                            InjectMode::Log,
                            InjectMode::Loopback,
                            InjectMode::Log];

        let result = from_str::<Vec<InjectMode>>(text);
        println!("{:?}", &result);
        let array = result.expect("Failed to deserialize a valid array of inject modes");
        assert_eq!(&expected, &array);
    }

    #[test]
    fn test_given_invalid_inject_mode_when_it_is_deserialized_then_we_get_the_right_result() {
        let text = r#"
        ["invalid inject mode", "log"]
        "#;

        let result = from_str::<Vec<InjectMode>>(text);
        println!("{:?}", &result);
        let _ = result.err().expect("Successfully deserialized an invalid inject mode");
    }

    #[test]
    fn test_given_message_when_it_contains_inject_mode_then_it_can_be_deserialized() {
        let text = r#"
        {
          "uuid": "UUID",
          "message": "message",
          "inject_mode": "forward"
        }
        "#;

        let expected_message = MessageActionBuilder::new("UUID", "message")
                                   .inject_mode(InjectMode::Forward)
                                   .build();
        let result = from_str::<MessageAction<String>>(text);
        let message = result.expect("Failed to deserialize a valid MessageAction object");
        assert_message_action_eq(&expected_message, &message);
    }

    #[test]
    fn test_given_message_is_deserialized_when_it_contains_an_unexpected_field_then_an_error_is_returned() {
        let text = r#"{ "unexpected": "UUID" }"#;
        let result = from_str::<MessageAction<String>>(text);
        let _ = result.err().unwrap();
    }

    #[test]
    fn test_given_message_without_message_field_when_it_is_deserialized_then_an_error_is_returned() {
        let text = r#"{ "uuid": "missing_msg" }"#;
        let result = from_str::<MessageAction<String>>(text);
        let _ = result.err().unwrap();
    }
}
