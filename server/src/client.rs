use std::net::SocketAddr;
use mio::net::TcpStream;
use crate::command::Command;
use ahash::AHashMap;
use crate::connection::Connection;

#[derive(Debug)]
pub(crate) struct Client {
    client_id: usize,
    address: SocketAddr,
    pub(crate) connection: Connection,
    cmd: Option<Command>,
}

impl Client {
    pub(crate) fn new(client_id: usize, conn: TcpStream, address: SocketAddr) -> Client {
        Self {
            client_id,
            address,
            connection: Connection::new(conn),
            cmd: Option::None
        }
    }

    pub(crate) fn read_from_query(&mut self) -> () {
        let protocol = match self.connection.read_protocol() {
            Some(frame) => frame,
            None => return (),
        };
        let command = Command::from(protocol);
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

    pub(crate) fn get_client(&mut self, client_id: usize) -> Option<&mut Box<Client>> {
        self.clients.get_mut(&client_id)
    }

    pub(crate) fn create_client(&mut self, fd: usize, conn: TcpStream, address: SocketAddr) -> () {
        let client = Box::new(Client::new(fd, conn, address));
        self.clients.insert(fd, client);
    }
}