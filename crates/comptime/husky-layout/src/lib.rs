mod query;

pub use query::*;

use std::alloc::{Layout, LayoutError};

pub fn repr_c(fields: &[Layout]) -> Result<(Layout, Vec<usize>), LayoutError> {
    let mut offsets = Vec::new();
    let mut layout = Layout::from_size_align(0, 1)?;
    for &field in fields {
        let (new_layout, offset) = layout.extend(field)?;
        layout = new_layout;
        offsets.push(offset);
    }
    // Remember to finalize with `pad_to_align`!
    Ok((layout.pad_to_align(), offsets))
}
