use std::io;
use std::time::Duration;
use core::lifecycle::lifecycle::ConstructiveLiteLifecycle;
use crate::eventloop::event::EventID;
use crate::server::{RedisServer, SERVER};

pub(crate) type IoEventProc<DATA: Sized + Send> = dyn FnOnce(DATA);
pub(crate) trait IoEventManager: ConstructiveLiteLifecycle {

    //int aeCreateFileEvent(aeEventLoop *eventLoop, int fd, int mask,
    //         aeFileProc *proc, void *clientData)
    fn create_io_event<DATA: Sized + Send>(&mut self, fd: i32, mask: i32, f: IoEventProc<DATA>)
        -> io::Result<()>;

    // void aeDeleteFileEvent(aeEventLoop *eventLoop, int fd, int mask)
    fn delete_io_event(&mut self, event_id: EventID) -> io::Result<()>;

    // There's no actual process io event in Redis, there's only process_events mix of process_time_events
    // and logically process_io_events
    fn process_io_events(&mut self, timeout: Option<Duration>) -> io::Result<i64>;

    fn get_server() -> *RedisServer {
        return &SERVER;
    }
}
