use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;
use mio::net::TcpStream;
use mio::Token;
use core::lifecycle::lifecycle::ConstructiveLiteLifecycle;
use crate::command::Command;
use ahash::AHashMap;
use mio::event::Event;
use tokio::net::unix::SocketAddr;
use crate::server::RedisServerConfig;

#[derive(Debug)]
pub(crate) struct Client {
    client_id: usize,
    address: SocketAddr,
    connection: TcpStream,
    cmd: Option<Command>,
}

impl Client {
    pub(crate) fn new(client_id: usize, conn: TcpStream, address: SocketAddr) -> Client {
        Self {
            client_id,
            address,
            connection: conn,
            cmd: Option::None
        }
    }

    pub(crate) fn read_from_query() -> () {

    }
}

impl Drop for Client {
    fn drop(&mut self) {
        todo!()
    }
}

impl core::lifecycle::lifecycle::LiteLifecycle for Client {
    fn initialize(&mut self) -> Result<(), Err> {
        todo!()
    }
}

type ClientID = usize;
pub(crate) struct ClientManager {
    clients: AHashMap<ClientID, Box<Client>>,
}

impl Default for ClientManager {
    fn default() -> Self {
        Self {
            clients: AHashMap::new()
        }
    }
}

impl ClientManager {
    pub(crate) fn create_client(client_id: usize, conn: TcpStream, address: SocketAddr) -> Option<Box<Client>> {
        let mut client = Box::new(Client::new(client_id, conn, address));
        return match client.initialize() {
            Err(e) => {
                log::error!("{}", e);
                None
            }
            () => Some(client)
        }
    }

    pub(crate) fn get_client(&mut self, client_id: usize) -> Option<&mut Box<Client>> {
        self.clients.get_mut(&client_id)
    }
}