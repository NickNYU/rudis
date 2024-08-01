use std::io;
use std::io::ErrorKind;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use mio::{Events, Interest, Poll, Token};
use mio::event::Event;
use mio::net::TcpListener;
use crate::client::{Client, ClientManager};
use crate::eventloop::io_event::IoEventManager;
use crate::server::RedisServer;

pub(crate) struct MioEventManager {
    mio_poll: Poll,

    events: Arc<Mutex<Events>>,

    binder: Arc<Mutex<TcpListener>>,

    id_generator: AtomicUsize,

    client_manager: Arc<Mutex<ClientManager>>,

}

impl MioEventManager {
    const ACCEPTOR: Token = Token(0);

    const EVENTS_SIZE: usize = 1024;

    pub(crate) fn new(redis_server: RedisServer) -> Self {
        let poll = Poll::new().unwrap();
        let add_str = format!("127.0.0.1:{port}", port = redis_server.config.port);
        let addr = add_str.parse().unwrap();
        let mut server = TcpListener::bind(addr).unwrap();
        // Register the server with poll we can receive events for it.
        poll.registry()
            .register(&mut server, Self::ACCEPTOR, Interest::READABLE)
            .expect("TODO: panic message");
        Self {
            mio_poll: poll,
            events: Arc::new(Mutex::new(Events::with_capacity(Self::EVENTS_SIZE))),
            binder: Arc::new(Mutex::new(server)),
            id_generator: AtomicUsize::new(1),
            client_manager: Arc::new(Mutex::new(redis_server.client_manager()))
        }
    }

    fn is_accept_event(event: &Event) -> bool {
        event.token() == Self::ACCEPTOR
    }

    fn accept_new_client(&self) -> () {
        if let Ok((mut connection, address)) = self.binder.lock().unwrap().accept() {
            println!("Accepted connection from: {}", address);
            let fd = self.id_generator.fetch_add(1, Ordering::Relaxed);
            self.mio_poll.registry().register(
                &mut connection,
                Token(fd),
                Interest::READABLE, ).expect("TODO: panic message");
            self.client_manager.lock().unwrap().create_client(fd, connection, address);
        };

    }

    fn read_for_client(&self, event: &Event) -> () {
        // event
        let mut binding = self.client_manager.lock().unwrap();
        let mut client = binding.get_client(event.token().0);
        match client {
            None => {unreachable!()}
            Some(mut c) => {
                match c.read_from_query() {
                    Ok(_) => {}
                    Err(_) => {binding.remove_client(event.token().0)}
                }
            }
        }

    }

    fn remove_client(&self, event: &Event) -> () {
        let mut binding = self.client_manager.lock().unwrap();
        binding.remove_client(event.token().0)
    }
}

impl IoEventManager for MioEventManager {

    fn process_io_events(&mut self, timeout: Option<Duration>) -> io::Result<i64> {
        let mut events = self.events.lock().unwrap();
        match self.mio_poll.poll(&mut *events, timeout) {
            Ok(()) => {
                let mut counter: i64 = 0;
                for mio_event in events.iter() {
                    if Self::is_accept_event(mio_event) {
                        self.accept_new_client();
                    } else if mio_event.is_readable() {
                        self.read_for_client(mio_event)
                    } else if mio_event.is_read_closed() || mio_event.is_write_closed() {
                        self.remove_client(mio_event);
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
