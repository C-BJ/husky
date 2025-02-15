use std::path::{Path, PathBuf};

use interner::{DefaultItd, Interner, IsInternPtr};
#[cfg(feature = "lsp_support")]
use lsp_types::Url;

type FileItdInner = DefaultItd<Path, PathBuf>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileItd(FileItdInner);
pub type FileInterner = Interner<FileItd>;

impl IsInternPtr for FileItd {
    type T = <FileItdInner as IsInternPtr>::T;

    type Owned = <FileItdInner as IsInternPtr>::Owned;

    fn new_intern_ptr(id: usize, target: &'static Self::T) -> Self {
        Self(<FileItdInner as IsInternPtr>::new_intern_ptr(id, target))
    }

    fn new_itr() -> Interner<Self> {
        Interner::new_empty()
    }
}
impl std::ops::Deref for FileItd {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::borrow::Borrow<Path> for FileItd {
    fn borrow(&self) -> &Path {
        &self.0
    }
}

pub trait InternFile {
    fn file_interner(&self) -> &FileInterner;

    fn intern_file(&self, path: PathBuf) -> FileItd {
        self.file_interner()
            .intern(match std::fs::canonicalize(path.clone()) {
                Ok(path) => path,
                Err(_) => path,
            })
    }

    #[cfg(feature = "lsp_support")]
    fn it_url(&self, url: &Url) -> Result<FileItd, ()> {
        Ok(self.intern_file(url.to_file_path()?))
    }
}

pub fn new_file_interner() -> FileInterner {
    FileInterner::new_empty()
}

#[test]
fn test_intern_file() {
    use husky_check_utils::*;
    let interner = new_file_interner();
    let path = &*interner.intern("haha".into());
    let path1: PathBuf = "haha".into();
    should_eq!(path, &path1);
}
