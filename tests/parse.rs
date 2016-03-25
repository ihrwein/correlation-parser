extern crate correlation_parser;
extern crate syslog_ng_common;
extern crate env_logger;

use correlation_parser::{CorrelationParserBuilder, options, CLASSIFIER_UUID, CLASSIFIER_CLASS};
use syslog_ng_common::{ParserBuilder, LogMessage, Parser};
use syslog_ng_common::mock::MockPipe;
use syslog_ng_common::sys::logmsg::log_msg_registry_init;

use std::thread;
use std::time::Duration;

#[test]
fn test_alert_is_forwarded() {
    let _ = env_logger::init();
    unsafe { log_msg_registry_init()};
    let mut logmsg = LogMessage::new();
    logmsg.insert(CLASSIFIER_UUID, "9cd7a5d6-d439-484d-95ac-7bf3bd055082");
    logmsg.insert(CLASSIFIER_CLASS, "LOGGEN");

    let config_file = "tests/contexts.json";
    let message = "seq: 0000000000, thread: 0000, runid: 1456947132, stamp: 2016-03-02T20:32:12 PAD";

    let mut pipe = MockPipe::new();
    let mut builder = CorrelationParserBuilder::<MockPipe>::new();
    builder.option(options::CONTEXTS_FILE.to_owned(), config_file.to_owned());
    let mut parser = builder.build().unwrap();
    assert_eq!(true, parser.parse(&mut pipe, &mut logmsg, message));
    assert_eq!(true, parser.parse(&mut pipe, &mut logmsg, message));
    assert_eq!(0, pipe.forwarded_messages.len());
    thread::sleep(Duration::from_secs(3));
    assert_eq!(0, pipe.forwarded_messages.len());
    thread::sleep(Duration::from_secs(2));
    // after 10 secs we should get one message when the parses next gets access to the pipe
    assert_eq!(true, parser.parse(&mut pipe, &mut logmsg, message));
    assert_eq!(1, pipe.forwarded_messages.len());
    let alert = pipe.forwarded_messages.get(0).unwrap();
    for i in alert.values() {
        println!("{:?}", i);
    }
    assert_eq!("Number of generated logs: 2", alert.get("MESSAGE"));
}
