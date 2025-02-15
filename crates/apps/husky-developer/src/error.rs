#[derive(Debug)]
pub struct DebuggerError {}
pub type DebuggerResult<T> = Result<T, DebuggerError>;

impl From<&'static str> for DebuggerError {
    fn from(_: &'static str) -> Self {
        todo!()
    }
}

impl From<std::io::Error> for DebuggerError {
    fn from(_: std::io::Error) -> Self {
        todo!()
    }
}

impl std::fmt::Display for DebuggerError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
