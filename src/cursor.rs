#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum CursorState {
    InCommandLine,
    Normal,
}

impl Default for CursorState {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Cursor {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) state: CursorState,
}
