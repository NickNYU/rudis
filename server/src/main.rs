use crate::ae::SingleThreadEventLoop;
use crate::server::RedisServer;

mod ae;
mod eventloop;
mod server;
mod client;
mod command;
mod connection;

fn main() {
    // SERVER.client_manager;
    let redis_server = RedisServer::default();
    SingleThreadEventLoop::new(redis_server).run();
}
