// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::sync::mpsc::Receiver;

use dispatcher::request::Request;
use reactor::EventDemultiplexer;
use Event;

pub struct Demultiplexer<T> {
    channel: Receiver<T>,
    stops: u32,
}

impl<T> Demultiplexer<T> {
    pub fn new(channel: Receiver<T>) -> Demultiplexer<T> {
        Demultiplexer {
            channel: channel,
            stops: 0,
        }
    }
}

impl<E: Event> EventDemultiplexer for Demultiplexer<Request<E>> {
    type Event = Request<E>;
    fn select(&mut self) -> Option<Self::Event> {
        let data = self.channel.recv().ok();

        if let Some(Request::Exit) = data {
            if self.stops >= 1 {
                None
            } else {
                self.stops += 1;
                Some(Request::Exit)
            }
        } else {
            data
        }
    }
}
