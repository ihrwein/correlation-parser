// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use config::{ContextConfigBuilder, ContextConfig};
use config::action::message::MessageActionBuilder;
use conditions::ConditionsBuilder;
use correlator::Correlator;
use context::ContextMap;
use message::MessageBuilder;
use action::Alert;
use Message;

use uuid::Uuid;
use serde_json::from_str;
use std::thread;
use std::time::Duration;

use test_utils::{MockAlertHandler, MockTemplate, MockTemplateFactory};

const JSON_CONFIG: &'static str = r#"
      [
        {
          "name": "CONTEXT_NAME_1",
          "uuid": "185e96da-c00e-454b-b4fe-9d0a14a86335",
          "patterns": [
            "p1",
            "p2",
            "p3"
          ],
          "conditions": {
            "timeout": 100,
            "first_opens": true
          },
          "actions": [
            {
              "message": {
                  "uuid": "uuid1",
                  "when": {
                    "on_opened": "true",
                    "on_closed": "true"
                  },
                  "message": "message_1"
              }
            }
          ]
        },
        {
          "name": "CONTEXT_NAME_2",
          "uuid": "285e96da-c00e-454b-b4fe-9d0a14a86335",
          "conditions": {
            "timeout": 10000,
            "max_size": 5
          },
          "actions": [
            {
              "message": {
                  "uuid": "uuid1",
                  "message": "message_2"
              }
            },
            {
              "message": {
                  "uuid": "uuid2",
                  "message": "message_2"
              }
            }
          ]
        },
        {
          "name": "CONTEXT_NAME_3",
          "uuid": "385e96da-c00e-454b-b4fe-9d0a14a86335",
          "patterns": [
            "p1"
          ],
          "conditions": {
            "timeout": 100
          },
          "actions": [
            {
              "message": {
                  "uuid": "uuid2",
                  "message": "message_3"
              }
            }
          ]
        }
      ]
    "#;

#[test]
fn test_given_manually_built_correlator_when_it_closes_a_context_then_the_actions_are_executed() {
    let uuid1 = "1b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_owned();
    let uuid2 = "2b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_owned();
    let uuid3 = "3b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_owned();
    let patterns = vec![
        uuid1.clone(),
        uuid2.clone(),
        uuid3.clone(),
    ];
    let condition = ConditionsBuilder::new(Duration::from_millis(100))
                        .first_opens(true)
                        .last_closes(true)
                        .build();
    let contexts = vec![
        ContextConfigBuilder::new(Uuid::new_v4(), condition.clone()).patterns(patterns.clone()).actions(vec![MessageActionBuilder::<String>::new("uuid", "message").build().into()]).build(),
        ContextConfigBuilder::new(Uuid::new_v4(), condition.clone()).patterns(patterns.clone()).actions(vec![MessageActionBuilder::<String>::new("uuid", "message").build().into()]).build(),
        ContextConfigBuilder::new(Uuid::new_v4(), condition.clone()).patterns(patterns.clone()).actions(vec![MessageActionBuilder::<String>::new("uuid", "message").build().into()]).build(),
    ];
    let mut responses = Vec::new();
    let alert_handler = Box::new(MockAlertHandler);
    let mut correlator: Correlator<Vec<Alert<Message>>, Message, MockTemplate> = Correlator::new(ContextMap::from_configs(contexts));
    correlator.set_alert_handler(Some(alert_handler));
    correlator.handle_events(&mut responses);
    let _ = correlator.push_message(MessageBuilder::new(&uuid1, "message").build());
    thread::sleep(Duration::from_millis(20));
    correlator.handle_events(&mut responses);
    let _ = correlator.push_message(MessageBuilder::new(&uuid2, "message").build());
    thread::sleep(Duration::from_millis(80));
    correlator.handle_events(&mut responses);
    let _ = correlator.push_message(MessageBuilder::new(&uuid3, "message").build());
    let _ = correlator.stop(&mut responses);
    assert_eq!(3, responses.len());
}

#[test]
fn test_given_correlator_when_it_is_built_from_json_then_we_get_the_expected_correlator() {
    let result = from_str::<Vec<ContextConfig<String>>>(JSON_CONFIG);
    let expected_name = "CONTEXT_NAME_1".to_owned();
    let expected_uuid = "185e96da-c00e-454b-b4fe-9d0a14a86335".to_owned();
    let mut contexts = result.expect("Failed to deserialize a config::ContextConfig from JSON");
    for i in &contexts {
        assert_eq!(true, i.name.is_some());
    }
    let context = contexts.remove(0);
    assert_eq!(Some(&expected_name), context.name.as_ref());
    assert_eq!(&expected_uuid, &context.uuid.to_hyphenated_string());
}

#[test]
fn test_given_correlator_when_it_is_built_from_json_then_it_produces_the_expected_number_of_messages
    () {
    let uuid1 = "1b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_owned();
    let uuid2 = "2b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_owned();
    let uuid3 = "3b47ba91-d867-4a8c-9553-a5dfd6ea1274".to_owned();
    let result = from_str::<Vec<ContextConfig<String>>>(JSON_CONFIG);
    let contexts = result.expect("Failed to deserialize a config::ContextConfig from JSON");
    let mut responses = Vec::new();
    let alert_handler = Box::new(MockAlertHandler);
    let mut correlator: Correlator<Vec<Alert<Message>>, Message, MockTemplate> = Correlator::new(ContextMap::from_configs(contexts));
    correlator.set_alert_handler(Some(alert_handler));
    correlator.handle_events(&mut responses);
    let _ = correlator.push_message(MessageBuilder::new(&uuid1, "message")
                                        .name(Some("p1"))
                                        .build());
    thread::sleep(Duration::from_millis(20));
    correlator.handle_events(&mut responses);
    let _ = correlator.push_message(MessageBuilder::new(&uuid2, "message")
                                        .name(Some("p2"))
                                        .build());
    correlator.handle_events(&mut responses);
    let _ = correlator.push_message(MessageBuilder::new(&uuid3, "message")
                                        .name(Some("p3"))
                                        .build());
    correlator.handle_events(&mut responses);
    let _ = correlator.push_message(MessageBuilder::new(&uuid3, "message")
                                        .name(Some("p3"))
                                        .build());
    correlator.handle_events(&mut responses);
    let _ = correlator.push_message(MessageBuilder::new(&uuid3, "message")
                                        .name(Some("p3"))
                                        .build());
    correlator.handle_events(&mut responses);
    let _ = correlator.push_message(MessageBuilder::new(&uuid3, "message")
                                        .name(Some("p3"))
                                        .build());
    correlator.handle_events(&mut responses);
    let _ = correlator.push_message(MessageBuilder::new(&uuid3, "message")
                                        .name(Some("p3"))
                                        .build());
    thread::sleep(Duration::from_millis(200));
    let _ = correlator.stop(&mut responses);
    println!("{:?}", &responses);
    assert_eq!(5, responses.len());
}
