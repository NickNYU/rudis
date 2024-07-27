
/* Process every pending time event, then every pending file event
 * (that may be registered by time event callbacks just processed).
 * Without special flags the function sleeps until some file event
 * fires, or when the next time event occurs (if any).
 *
 * If flags is 0, the function does nothing and returns.
 * if flags has AE_ALL_EVENTS set, all the kind of events are processed.
 * if flags has AE_FILE_EVENTS set, file events are processed.
 * if flags has AE_TIME_EVENTS set, time events are processed.
 * if flags has AE_DONT_WAIT set, the function returns ASAP once all
 * the events that can be handled without a wait are processed.
 * if flags has AE_CALL_AFTER_SLEEP set, the aftersleep callback is called.
 * if flags has AE_CALL_BEFORE_SLEEP set, the beforesleep callback is called.
 *
 * The function returns the number of events processed. */
pub(crate) type EventLoopFlag = i32;


const AE_FILE_EVENTS: i32 = 1<<0;
const AE_TIME_EVENTS: i32 = 1<<1;
const AE_ALL_EVENTS: i32 = AE_FILE_EVENTS | AE_TIME_EVENTS;
const AE_DONT_WAIT: i32 = 1<<2;
const AE_CALL_BEFORE_SLEEP: i32 = 1<<3;
const AE_CALL_AFTER_SLEEP: i32 = 1<<4;

pub(crate) fn is_file_event(flags: EventLoopFlag) -> bool {
    flags & AE_FILE_EVENTS > 0
}

pub(crate) fn is_time_event(flags: EventLoopFlag) -> bool {
    flags & AE_TIME_EVENTS > 0
}

pub(crate) fn is_all_event(flags: EventLoopFlag) -> bool {
    flags & AE_ALL_EVENTS > 0
}

pub(crate) fn is_dont_wait(flags: EventLoopFlag) -> bool {
    flags & AE_DONT_WAIT > 0
}

pub(crate) fn is_call_after_sleep(flags: EventLoopFlag) -> bool {
    flags & AE_CALL_AFTER_SLEEP > 0
}

pub(crate) fn is_call_before_sleep(flags: EventLoopFlag) -> bool {
    flags & AE_CALL_BEFORE_SLEEP > 0
}