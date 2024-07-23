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
    fn create_time_event<DATA: Sized + Send>(&mut self, duration: Duration, f: TimeProc<DATA>)
        -> io::Result<EventID>;

    // int aeDeleteTimeEvent(aeEventLoop *eventLoop, long long id)
    fn delete_time_event(&mut self, event_id: EventID) -> io::Result<()>;

    // static int processTimeEvents(aeEventLoop *eventLoop)
    fn process_time_events(&mut self) -> io::Result<i64>;
}