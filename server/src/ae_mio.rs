use std::sync::Arc;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use bytes::{BytesMut, BufMut};
use crate::ae::EventLoop;


struct MioEventLoop {

    mio_poll: Poll,

    events: Events
}

impl MioEventLoop {
    const SERVER: Token = Token(0);

    const EVENTS_SIZE: usize = 1024;
}

impl EventLoop for MioEventLoop {

}

impl core::lifecycle::lifecycle::ConstructiveLifecycle for MioEventLoop {}

impl core::lifecycle::construct::Constructive for MioEventLoop {
    type Instance = MioEventLoop;

    fn new() -> Self::Instance {
        let poll = Poll::new().unwrap();
        Self {
            mio_poll: poll,
            events: Events::with_capacity(Self::EVENTS_SIZE),
        }
    }
}

impl core::lifecycle::lifecycle::Lifecycle for MioEventLoop {
    fn initialize() -> Result<(), Err> {
        todo!()
    }

    fn start() -> Result<(), Err> {
        todo!()
    }

    fn stop() -> Result<(), Err> {
        todo!()
    }
}

impl Drop for MioEventLoop {
    fn drop(&mut self) {
        todo!()
    }
}
