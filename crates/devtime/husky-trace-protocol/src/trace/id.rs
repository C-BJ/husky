use super::*;
use husky_signal::Signalable;

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Hash)]
pub struct TraceId(usize);

impl std::fmt::Display for TraceId {
    fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
        write!(f, "tr#{}", self.0)
    }
}

impl Signalable for TraceId {}

impl TraceId {
    pub fn raw(self) -> usize {
        self.0
    }

    pub fn new(raw: usize) -> Self {
        assert!(raw < 10000); // ad hoc
        TraceId(raw)
    }
}
