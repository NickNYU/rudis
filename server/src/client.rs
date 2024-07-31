use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use mio::net::TcpStream;
use crate::command::Command;
use ahash::AHashMap;
use crate::connection::Connection;

#[derive(Debug, Clone)]
pub(crate) struct Client {
    client_id: usize,
    address: SocketAddr,
    pub(crate) connection: Arc<Mutex<Connection>>,
    // cmd: Option<Command>,
}

impl Client {
    pub(crate) fn new(client_id: usize, conn: TcpStream, address: SocketAddr) -> Client {
        Self {
            client_id,
            address,
            connection: Arc::new(Mutex::new(Connection::new(conn))),
            // cmd: None
        }
    }

    pub(crate) fn read_from_query(&mut self) -> () {
        let protocol = match self.connection.lock().unwrap().read_protocol() {
            Ok(op_protocol) => op_protocol,
            Err(e) => return (),
        };
        if let Some(prot) = protocol {
            let command = Command::from_protocol(prot).unwrap();
            command.apply(&mut self.connection.lock().unwrap()).unwrap()
        }
    }
}

// impl Drop for Client {
//     fn drop(&mut self) {
//         todo!()
//     }
// }


type ClientID = usize;
#[derive(Debug, Clone)]
pub(crate) struct ClientManager {
    clients: Arc<Mutex<AHashMap<ClientID, Box<Client>>>>,
}

impl Default for ClientManager {
    fn default() -> Self {
        Self {
            clients: Arc::new(Mutex::new(AHashMap::new()))
        }
    }
}

impl ClientManager {

    pub(crate) fn get_client(&mut self, client_id: usize) -> Box<Client> {
        let mut binding = self.clients.lock().unwrap();
        let client = binding.get_mut(&client_id);
        client.unwrap().clone()
    }

    pub(crate) fn create_client(&mut self, fd: usize, conn: TcpStream, address: SocketAddr) -> () {
        let client = Box::new(Client::new(fd, conn, address));
        self.clients.lock().unwrap().insert(fd, client);
    }
}