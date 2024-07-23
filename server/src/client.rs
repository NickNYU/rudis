use mio::net::TcpStream;
use core::lifecycle::lifecycle::LiteLifecycle;

pub(crate) struct Client {
    connection: TcpStream,
    cmd:
}

impl Default for Client {
    fn default() -> Self {
        todo!()
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        todo!()
    }
}

impl LiteLifecycle for Client {
    fn initialize() -> Result<(), Err> {
        todo!()
    }
}

pub(crate) struct ClientManager {

}

impl Default for ClientManager {
    fn default() -> Self {
        todo!()
    }
}

impl ClientManager {
    pub fn create_client() -> Client {
        let client = Client::default();
        match client.initialize() {
            Err(e) => log::error!("{}", e)
        }
        client
    }
}