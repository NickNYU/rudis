use crate::lifecycle::construct::Constructive;

pub trait Lifecycle {

}

pub trait ConstructiveLifecycle: Constructive + Lifecycle {

}

pub trait LiteLifecycle {
}

pub trait ConstructiveLiteLifecycle: Constructive + LiteLifecycle {

}