use crate::lifecycle::construct::Constructive;

pub trait Lifecycle: Drop {

    fn initialize() -> Result<(), Err>;
    fn start() -> Result<(), Err>;

    fn stop() -> Result<(), Err>;
}

pub trait ConstructiveLifecycle: Constructive + Lifecycle {

}

pub trait LiteLifecycle: Drop {
    fn initialize() -> Result<(), Err>;
}

pub trait ConstructiveLiteLifecycle: Constructive + LiteLifecycle {

}