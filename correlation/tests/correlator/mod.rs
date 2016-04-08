// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use correlation::Alert;
use correlation::correlator::{Correlator, CorrelatorFactory, Error};
use correlation::{MessageBuilder, Message};
use correlation::test_utils::MockAlertHandler;

use env_logger;

#[test]
fn test_given_correlator_when_messages_are_received_then_they_are_grouped_into_a_context_by_a_context_id
    () {
    let _ = env_logger::init();
    let contexts_file = "tests/correlator/contexts.json";
    let mut responses = Vec::new();
    let mut correlator: Correlator<Vec<Alert<Message>>, Message> = CorrelatorFactory::from_path(contexts_file)
                             .ok()
                             .expect("Failed to load contexts from a valid contexts_file");
    let login_message = MessageBuilder::new("6d2cba0c-e241-464a-89c3-8035cac8f73e", "message")
                            .name(Some("LOGIN"))
                            .pair("user_name", "linus")
                            .build();
    let read_message = MessageBuilder::new("60dd1233-5fa6-4e3b-993f-e04ef9b4c164", "message")
                           .name(Some("MAIL_READ"))
                           .pair("user_name", "linus")
                           .build();
    let logout_message = MessageBuilder::new("91ea534a-4880-4853-aec2-7b2a2df9a8c9", "message")
                             .name(Some("LOGOUT"))
                             .pair("user_name", "linus")
                             .build();
    let alert_handler = Box::new(MockAlertHandler);
    correlator.set_alert_handler(Some(alert_handler));
    let _ = correlator.push_message(login_message);
    let _ = correlator.push_message(read_message);
    let _ = correlator.push_message(logout_message);
    let _ = correlator.stop(&mut responses);
    assert_eq!(1, responses.len());
}

#[test]
fn test_given_correlator_factory_when_the_config_file_does_not_exist_then_it_returns_io_error() {
    let _ = env_logger::init();
    let contexts_file = "not_existing_file.json";
    let result: Result<Correlator<(), Message>, Error> = CorrelatorFactory::from_path(contexts_file);
    if let Error::Io(_) = result.err().unwrap() {
    } else {
        unreachable!();
    }
}

#[test]
fn test_given_correlator_factory_when_it_reads_an_invalid_config_then_it_returns_deser_error() {
    let _ = env_logger::init();
    let contexts_file = "tests/correlator/invalid.json";
    let result: Result<Correlator<(), Message>, _> = CorrelatorFactory::from_path(contexts_file);
    if let Error::SerdeJson(_) = result.err().unwrap() {
    } else {
        unreachable!();
    }
}

#[test]
fn test_given_yaml_context_file_when_it_is_read_by_the_correlator_factory_then_the_contexts_are_deserialized() {
    let _ = env_logger::init();
    let contexts_file = "tests/correlator/contexts.yaml";
    let result = CorrelatorFactory::load_file(contexts_file).unwrap();
    assert_eq!(1, result.len());
}

#[test]
fn test_name() {
    let _ = env_logger::init();
    let contexts_file = "tests/correlator/invalid.yaml";
    let result = CorrelatorFactory::load_file(contexts_file);
    if let Error::SerdeYaml(_) = result.err().unwrap() {
    } else {
        unreachable!();
    }
}
