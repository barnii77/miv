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

impl Cursor {
    fn left(&mut self, ed_state: &mut crate::editor_state::EditorState) {
        if self.x > 0 {
            self.x -= 1;
        }
        ed_state.get_buffer_mut().content.move_gap(self.x - 1);
    }
}
