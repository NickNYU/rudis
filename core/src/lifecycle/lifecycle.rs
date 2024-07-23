use crate::lifecycle::construct::Constructive;

pub trait Lifecycle: Drop {

    fn initialize(&mut self) -> Result<(), Err>;
    fn start(&mut self) -> Result<(), Err>;

    fn stop(&mut self) -> Result<(), Err>;
}

pub trait ConstructiveLifecycle: Constructive + Lifecycle {

}

pub trait LiteLifecycle: Drop {
    fn initialize(&mut self) -> Result<(), Err>;
}

pub trait ConstructiveLiteLifecycle: Constructive + LiteLifecycle {

}