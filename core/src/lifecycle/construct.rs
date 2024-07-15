pub trait Constructive {
    type Instance;

    fn new() -> Self::Instance;
}