use crate::{line_token_iter::LineTokenIter, *};

use husky_dev_utils::dev_src;
use husky_file::{FileError, FileErrorKind, FileResultArc};
#[salsa::query_group(TokenQueryGroupStorage)]
pub trait TokenizedTextQueryGroup: husky_file::FileQueryGroup + TokenQueryGroup {
    fn tokenized_text(&self, id: husky_file::FilePtr) -> FileResultArc<TokenizedText>;
}

fn tokenized_text(
    this: &dyn TokenizedTextQueryGroup,
    id: husky_file::FilePtr,
) -> FileResultArc<TokenizedText> {
    if let Some(text) = this.raw_text(id) {
        return Ok(TokenizedText::parse(this.word_allocator(), text.as_str()));
    } else {
        Err(FileError {
            kind: FileErrorKind::FileNotFound,
            dev_src: dev_src!(),
        })
    }
}

pub trait TokenQueryGroup: husky_word::InternWord {
    fn tokenize(&self, line: &str) -> Vec<HuskyToken> {
        LineTokenIter::new(
            self.word_allocator(),
            0,
            line.chars().enumerate().peekable(),
        )
        .1
        .collect()
    }
}
