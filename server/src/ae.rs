use std::io;
use std::time::Duration;
use crate::eventloop::io_event::IoEventManager;
use crate::eventloop::mio_event_manager::MioEventManager;

pub(crate) trait EventLoop {
    fn get_max_file_descriptor() -> i32;

    fn before_sleep(&mut self) -> io::Result<()>;

    fn after_sleep(&mut self) -> io::Result<()>;

}

pub(crate) struct SingleThreadEventLoop {
    io_event_loop: MioEventManager,
}

impl Default for SingleThreadEventLoop {
    fn default() -> Self {
        Self {
            io_event_loop: MioEventManager::new()
        }
    }
}
impl SingleThreadEventLoop {
    pub(crate) fn run(&mut self) -> () {
        loop {
            // self.before_sleep().unwrap();
            self.io_event_loop
                .process_io_events(Some(Duration::from_secs(1)))
                .expect("no io event");
            // self.after_sleep().unwrap();
        }
    }
}


impl core::lifecycle::construct::Constructive for SingleThreadEventLoop {
    type Instance = ();

    fn new() -> Self::Instance {
        todo!()
    }
}


impl Drop for SingleThreadEventLoop {
    fn drop(&mut self) {
        todo!()
    }
}




