use std::net::SocketAddr;
use std::sync::Arc;
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
            cmd: None
        }
    }

    pub(crate) fn read_from_query(&mut self) -> () {
        let protocol = match self.connection.read_protocol() {
            Ok(op_protocol) => op_protocol.unwrap(),
            Err(e) => return (),
        };
        let command = Command::from_protocol(protocol).unwrap();
        command.apply(&mut self.connection).unwrap()
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        todo!()
    }
}


type ClientID = usize;
#[derive(Debug, Clone)]
pub(crate) struct ClientManager {
    clients: Arc<AHashMap<ClientID, Box<Client>>>,
}

impl Default for ClientManager {
    fn default() -> Self {
        Self {
            clients: Arc::new(AHashMap::new())
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