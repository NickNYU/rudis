use mio::net::TcpStream;
use mio::Token;
use core::lifecycle::lifecycle::ConstructiveLiteLifecycle;
use crate::command::Command;
use ahash::AHashMap;
use mio::event::Event;
use crate::server::RedisServerConfig;

#[derive(Debug)]
pub(crate) struct Client {
    client_id: i64,
    connection: TcpStream,
    cmd: Option<Command>,
}

impl Client {
    pub(crate) fn new(client_id: i64) -> Box<Client> {

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

pub(crate) struct ClientManager {
    clients: AHashMap<Token, Box<Client>>
}

impl Default for ClientManager {
    fn default() -> Self {
        Self {
            clients: AHashMap::new(),
        }
    }
}

impl ClientManager {
    pub fn create_client(client_id: i64) -> Client {
        let client = Client::default();
        match client.initialize() {
            Err(e) => log::error!("{}", e)
        }
        client
    }

    pub(crate) fn get_client() -> Client {

    }
}