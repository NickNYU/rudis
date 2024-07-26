use std::io;
use std::time::Duration;
use core::lifecycle::lifecycle::ConstructiveLiteLifecycle;

pub(crate) type IoEventProc<DATA: Sized + Send> = dyn FnOnce(DATA);
pub(crate) trait IoEventManager: ConstructiveLiteLifecycle {


    // There's no actual process io event in Redis, there's only process_events mix of process_time_events
    // and logically process_io_events
    fn process_io_events(&mut self, timeout: Option<Duration>) -> io::Result<i64>;


}
