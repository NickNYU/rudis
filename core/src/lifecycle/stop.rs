
pub(crate) trait Stoppable {
    fn stop() -> Result<()>;
}