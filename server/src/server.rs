use std::sync::Arc;
use crate::client::ClientManager;

#[derive(Debug, Clone)]
pub(crate) struct RedisServer {
    pub(crate) client_manager: ClientManager,
    pub(crate) config: Arc<RedisServerConfig>
}

impl Default for RedisServer {
    fn default() -> Self {
        Self {
            client_manager: ClientManager::default(),
            config: Arc::new(RedisServerConfig{port: 6379})
        }
    }
}

impl RedisServer {
    pub fn client_manager(&self) -> ClientManager {
        self.client_manager.clone()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RedisServerConfig {
    pub(crate) port: i32,

}