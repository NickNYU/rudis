use crate::lifecycle::{initialize::Initializable, start::Startable, stop::Stoppable};
use crate::lifecycle::construct::Constructive;

pub trait Lifecycle: Initializable + Startable + Stoppable + Drop {

}

pub trait ConstructiveLifecyle: Constructive + Lifecycle {

}

pub trait LiteLifecycle: Initializable + Drop {

}

pub trait ConstructiveLiteLifecyle: Constructive + LiteLifecycle {

}