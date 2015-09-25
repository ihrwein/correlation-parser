use dispatcher::request::InternalRequest;

pub use self::linear::LinearContext;
pub use self::map::MapContext;
pub use self::base::BaseContext;
pub use self::base::BaseContextBuilder;

pub mod base;
pub mod event;
pub mod map;
#[cfg(test)]
mod test;

pub enum Context {
    Linear(LinearContext),
    Map(MapContext)
}

impl From<Context> for Box<self::event::EventHandler<InternalRequest>> {
    fn from(context: Context) -> Box<self::event::EventHandler<InternalRequest>> {
        match context {
            Context::Linear(context) => Box::new(context),
            Context::Map(context) => Box::new(context),
        }
    }
}

pub mod linear {
    use uuid::Uuid;
    use std::rc::Rc;

    use conditions::Conditions;
    use context::event::{EventHandler};
    use message::{Message};
    use state::State;
    use timer::TimerEvent;
    use dispatcher::request::{InternalRequest, Request};
    use context::base::{
        BaseContext,
        BaseContextBuilder
    };

    pub struct LinearContext {
        base: BaseContext,
        state: State
    }

    impl LinearContext {
        pub fn new(uuid: Uuid, conditions: Conditions) -> LinearContext {
            LinearContext {
                base: BaseContextBuilder::new(uuid, conditions).build(),
                state: State::new()
            }
        }

        pub fn on_event(&mut self, event: InternalRequest) {
            trace!("LinearContext: received event");
            match event {
                Request::Timer(event) => {
                    self.on_timer(&event)
                },
                Request::Message(message) => {
                    self.on_message(message)
                },
                _ => {}
            }
        }

        pub fn on_timer(&mut self, event: &TimerEvent) {
            self.base.on_timer(event, &mut self.state)
        }

        pub fn on_message(&mut self, event: Rc<Message>) {
            self.base.on_message(event, &mut self.state);
        }

        pub fn is_open(&self) -> bool {
            self.state.is_open()
        }

        pub fn patterns(&self) -> &[String] {
            &self.base.conditions().patterns
        }
    }

    impl From<BaseContext> for LinearContext {
        fn from(context: BaseContext) -> LinearContext {
            LinearContext {
                base: context,
                state: State::new()
            }
        }
    }

    impl EventHandler<InternalRequest> for LinearContext {
        fn handlers(&self) -> &[String] {
            self.patterns()
        }
        fn handle_event(&mut self, event: InternalRequest) {
            self.on_event(event);
        }
    }
}
