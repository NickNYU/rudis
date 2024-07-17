use std::io;
use std::time::Duration;
use crate::eventloop::event::EventID;
use core::lifecycle::lifecycle::ConstructiveLiteLifecycle;


type TimeProc<DATA: Sized + Send> = dyn FnOnce(DATA);
pub(crate) trait TimeEventManager: ConstructiveLiteLifecycle {
    //Redis code is as
    // ```c
    // long long aeCreateTimeEvent(aeEventLoop *eventLoop, long long milliseconds,
    //         aeTimeProc *proc, void *clientData,
    //         aeEventFinalizerProc *finalizerProc)
    // ```
    fn create_time_event<DATA: Sized + Send>(duration: Duration, f: TimeProc<DATA>) -> io::Result<EventID>;

    fn delete_time_event(event_id: EventID) -> io::Result<()>;

    fn process_time_event() -> io::Result<i64>;
}