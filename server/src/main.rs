use crate::ae::SingleThreadEventLoop;
use crate::server::SERVER;

mod ae;
mod eventloop;
mod server;
mod client;
mod command;
mod connection;

fn main() {
    // SERVER.client_manager;
    SingleThreadEventLoop::default().run();
}
