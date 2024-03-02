use std::path::PathBuf;

#[derive(Clone)]
pub(crate) struct Buffer {
    pub(crate) content: crate::gap_buffer::GapBuffer<char>,
    pub(crate) name: String,
    pub(crate) location: Option<PathBuf>,
}

impl Buffer {
    pub(crate) fn new() -> Self {
        Self {
            content: crate::gap_buffer::GapBuffer::<char>::new_empty(),
            name: String::new(),
            location: None,
        }
    }
}
