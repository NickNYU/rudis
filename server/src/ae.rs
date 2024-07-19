use std::io;
use core::lifecycle::lifecycle::{ConstructiveLifecycle};
use crate::eventloop::eventloop::EventLoopFlag;

pub trait EventLoop: ConstructiveLifecycle {
    fn get_max_file_descriptor() -> i32;

    fn before_sleep(self) -> io::Result<()>;

    fn after_sleep(self) -> io::Result<()>;

    fn process_events(self, flags: EventLoopFlag) -> io::Result<()> {

    }

    fn time_event_manager() -> TimeEventManager;
}



