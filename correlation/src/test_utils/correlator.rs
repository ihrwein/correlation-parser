// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::sync::mpsc::Sender;

use dispatcher::request::Request;
use action::Alert;
use correlator::AlertHandler;
use Event;

pub struct MockAlertHandler;

impl<E: Event> AlertHandler<Vec<Alert<E>>, E> for MockAlertHandler {
    fn on_alert(&mut self, alert: Alert<E>, _: &mut Sender<Request<E>>, extra_data: &mut Vec<Alert<E>>) {
        extra_data.push(alert);
    }
}
