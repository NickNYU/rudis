use crate::client::ClientManager;

pub static SERVER: RedisServer = RedisServer::default();
pub(crate) struct RedisServer {
    client_manager: ClientManager,
    pub config: RedisServerConfig
}

impl Default for RedisServer {
    fn default() -> Self {
        Self {
            client_manager: ClientManager::default(),
            config: RedisServerConfig{port: 6379}
        }
    }
}

impl RedisServer {
    pub fn get_client_manager(self) -> ClientManager {
        self.client_manager
    }
}

pub(crate) struct RedisServerConfig {
    pub(crate) port: i32,

}