use husky_display_utils::{HuskyDisplay, HuskyDisplayConfig};
use husky_file::{FileItd, InternFile};
#[cfg(feature = "lsp_support")]
use lsp_types::TextDocumentPositionParams;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

use crate::*;

#[derive(
    Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Serialize, Deserialize,
)]
pub struct TextPosition {
    pub row: Row,
    pub col: Column,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct FilePosition {
    file: FileItd,
    pos: TextPosition,
}

impl FilePosition {
    pub fn file(&self) -> FileItd {
        self.file
    }

    pub fn pos(&self) -> TextPosition {
        self.pos
    }
}

#[cfg(feature = "lsp_support")]
impl From<lsp_types::Position> for TextPosition {
    fn from(pos: lsp_types::Position) -> Self {
        Self {
            row: Row(pos.line),
            col: Column(pos.character),
        }
    }
}

#[cfg(feature = "lsp_support")]
impl FilePosition {
    pub fn from_proto(db: &dyn InternFile, doc_pos: &TextDocumentPositionParams) -> Self {
        let file = db.it_url(&doc_pos.text_document.uri).expect("todo");
        let pos: TextPosition = doc_pos.position.into();
        Self { file, pos }
    }
}

impl HuskyDisplay for TextPosition {
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
        if config.colored {
            write!(
                result,
                "{}pos {: <4?}{}",
                husky_print_utils::YELLOW,
                self,
                husky_print_utils::RESET
            )
            .unwrap();
        } else {
            write!(result, "pos {: <4?}", self,).unwrap();
        }
    }
}

impl From<(u32, u32)> for TextPosition {
    fn from(pair: (u32, u32)) -> Self {
        TextPosition {
            row: pair.0.into(),
            col: pair.1.into(),
        }
    }
}

impl From<(usize, usize)> for TextPosition {
    fn from(pair: (usize, usize)) -> Self {
        TextPosition {
            row: pair.0.into(),
            col: pair.1.into(),
        }
    }
}

impl From<(i32, i32)> for TextPosition {
    fn from(pair: (i32, i32)) -> Self {
        TextPosition {
            row: pair.0.into(),
            col: pair.1.into(),
        }
    }
}

impl TextPosition {
    pub fn line(&self) -> u32 {
        self.row.0 + 1
    }

    pub fn i(&self) -> u32 {
        self.row.0
    }
    pub fn j(&self) -> u32 {
        self.col.0
    }

    pub fn to_left(&self, x: u32) -> TextPosition {
        Self {
            row: self.row,
            col: Column(self.col.0 - x),
        }
    }

    pub fn to_right(&self, x: u32) -> TextPosition {
        Self {
            row: self.row,
            col: Column(self.col.0 + x),
        }
    }
}

impl std::fmt::Display for TextPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}:{:?}", self.row.0 + 1, self.col.0 + 1))
    }
}

impl Into<lsp_types::Position> for TextPosition {
    fn into(self) -> lsp_types::Position {
        lsp_types::Position::new(self.row.0, self.col.0)
    }
}
