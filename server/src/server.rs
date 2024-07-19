use crate::client::ClientManager;

pub static SERVER: RedisServer = RedisServer::default();
pub(crate) struct RedisServer {
    client_manager: ClientManager
}

impl Default for RedisServer {
    fn default() -> Self {
        Self {
            client_manager: ClientManager::default()
        }
    }
}

impl RedisServer {
    pub fn get_client_manager(self) -> ClientManager {
        self.client_manager
    }
}