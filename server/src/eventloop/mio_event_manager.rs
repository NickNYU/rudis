use std::io;
use std::io::ErrorKind;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use mio::{Events, Interest, Poll, Token};
use mio::event::Event;
use mio::net::TcpListener;
use crate::eventloop::io_event::IoEventManager;
use crate::server::SERVER;

pub(crate) struct MioEventManager {
    mio_poll: Poll,

    events: Events,

    binder: Arc<Mutex<TcpListener>>,

    id_generator: AtomicUsize,
}

impl MioEventManager {
    const ACCEPTOR: Token = Token(0);

    const EVENTS_SIZE: usize = 1024;

    pub(crate) fn new() -> Self {
        let poll = Poll::new().unwrap();
        let add_str = format!("127.0.0.1:{port}", port=SERVER.config.port);
        let addr = add_str.parse().unwrap();
        let mut server = TcpListener::bind(addr).unwrap();
        // Register the server with poll we can receive events for it.
        poll.registry()
            .register(&mut server, Self::ACCEPTOR, Interest::READABLE)
            .expect("TODO: panic message");
        Self {
            mio_poll: poll,
            events: Events::with_capacity(Self::EVENTS_SIZE),
            binder: Arc::new(Mutex::new(server)),
            id_generator: AtomicUsize::new(1),
        }
    }

    fn is_accept_event(&self, event: &Event) -> bool {
        event.token() == Self::ACCEPTOR
    }

    fn accept_new_client(&self) -> () {
        if let Ok((mut connection, address)) = self.binder.as_ref().get_mut().unwrap().accept() {
            println!("Accepted connection from: {}", address);
            let fd = self.id_generator.fetch_add(1, Ordering::Relaxed);
            self.mio_poll.registry().register(
                &mut connection,
                Token(fd),
                Interest::READABLE.add(Interest::WRITABLE), ).expect("TODO: panic message");
            SERVER.client_manager.create_client(fd, connection, address);
        };

    }

    fn read_for_client(&self, event: &Event) -> () {
        // event
        if let Some(client) = SERVER
            .client_manager.get_client(event.token().0) {
            client.read_from_query();
        }
    }
}

impl IoEventManager for MioEventManager {

    fn process_io_events(&mut self, timeout: Option<Duration>) -> io::Result<i64> {
        match self.mio_poll.poll(&mut self.events, timeout) {
            Ok(()) => {
                let mut counter: i64 = 0;
                for mio_event in &self.events {
                    if self.is_accept_event(mio_event) {
                        self.accept_new_client();
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
