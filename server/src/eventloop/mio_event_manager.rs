use std::io;
use std::io::ErrorKind;
use std::time::Duration;
use log::error;
use mio::{Events, Interest, Poll, Token};
use mio::event::Event;
use mio::net::TcpListener;
use crate::eventloop::event::EventID;
use crate::eventloop::io_event::IoEventManager;
use crate::server::SERVER;

pub(crate) struct MioEventManager {
    mio_poll: Poll,

    events: Events
}

impl MioEventManager {
    const ACCEPTOR: Token = Token(0);

    const EVENTS_SIZE: usize = 1024;

    fn is_accept_event(&self, event: &Event) -> bool {
        event.token() == Self::ACCEPTOR
    }

    fn accept_new_client(&self, event: &Event) {
        self.get_server().get_client_manager().create_client();
    }

    fn read_for_client(&self, event: &Event) {
        // event
    }
}

impl IoEventManager for MioEventManager {
    fn create_io_event<DATA: Sized + Send>(&mut self, fd: i32, mask: i32, f: Box<crate::eventloop::io_event::IoEventProc<DATA>>) -> std::io::Result<()> {
        todo!()
    }

    fn delete_io_event(&mut self, event_id: EventID) -> std::io::Result<()> {
        todo!()
    }

    fn process_io_events(&mut self, timeout: Option<Duration>) -> io::Result<i64> {
        match self.mio_poll.poll(&mut self.events, timeout) {
            Ok(()) => {
                let mut counter: i64 = 0;
                for mio_event in &self.events {
                    if self.is_accept_event(mio_event) {
                        self.accept_new_client(mio_event);
                    } else {
                        self.read_for_client(mio_event)
                    }
                    counter += 1;
                }
                Ok(counter)
            }
            Err(ref err) if err.kind() == ErrorKind::Interrupted => Ok(0),
            Err(ref err) => panic!("{}: No error here", err),
        }
    }
}

impl core::lifecycle::lifecycle::ConstructiveLiteLifecycle for MioEventManager {}

impl core::lifecycle::construct::Constructive for MioEventManager {
    type Instance = MioEventManager;

    fn new() -> Self::Instance {
        let poll = Poll::new().unwrap();
        Self {
            mio_poll: poll,
            events: Events::with_capacity(Self::EVENTS_SIZE),
        }
    }
}

impl core::lifecycle::lifecycle::LiteLifecycle for MioEventManager {
    fn initialize(&mut self) -> Result<(), Err> {
        // Setup the TCP server socket.
        let add_str = format!("127.0.0.1:{port}", port=SERVER.config.port);
        let addr = add_str.parse().unwrap();
        let mut server = TcpListener::bind(addr)?;
        // Register the server with poll we can receive events for it.
        self.mio_poll.registry()
            .register(&mut server, Self::ACCEPTOR, Interest::READABLE)?;
    }
}

impl Drop for MioEventManager {
    fn drop(&mut self: Self::Instance) {
        drop(self.events);
        drop(self.mio_poll);
        // todo!()
    }
}