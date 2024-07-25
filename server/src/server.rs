use lazy_static::lazy_static;
use crate::client::ClientManager;

lazy_static! {
    pub static ref SERVER: Box<RedisServer> = Box::new(RedisServer::default());
}
pub(crate) struct RedisServer {
    pub(crate) client_manager: ClientManager,
    pub(crate) config: RedisServerConfig
}

impl Default for RedisServer {
    fn default() -> Self {
        Self {
            client_manager: ClientManager::default(),
            config: RedisServerConfig{port: 6379}
        }
    }
}

pub(crate) struct RedisServerConfig {
    pub(crate) port: i32,

}