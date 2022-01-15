use crate::*;

use super::*;

// inner ops
impl<'a> AtomLRParser<'a> {
    pub(crate) fn push(&mut self, kind: AtomKind) -> AstResult<()> {
        self.stack.push(Atom::new(self.stream.pop_range(), kind))
    }

    pub(crate) fn save_stream(&self) -> Stream<'a> {
        self.stream.clone()
    }

    pub(crate) fn rollback(&mut self, stream: Stream<'a>) {
        self.stream = stream
    }
}
