pub(crate) enum EditorMode {
    Normal,
    Insert,
    Visual { cursor_start: crate::cursor::Cursor },
}

#[derive(Default)]
pub(crate) struct CommandLine {
    pub(crate) buffer: String,
}

impl EditorMode {
    pub(crate) fn new_normal() -> Self {
        Self::Normal
    }

    pub(crate) fn new_insert() -> Self {
        Self::Insert
    }

    pub(crate) fn new_visual(cursor_start: crate::cursor::Cursor) -> Self {
        Self::Visual { cursor_start }
    }
}

pub(crate) struct TermInfo {
    pub(crate) rows: u16,
    pub(crate) cols: u16,
}

pub(crate) struct EditorGlobals {
    pub(crate) normal_mode_motion_tree: crate::motion_interpreter::MotionTree,
    pub(crate) insert_mode_motion_tree: crate::motion_interpreter::MotionTree,
    pub(crate) visual_mode_motion_tree: crate::motion_interpreter::MotionTree,
    pub(crate) tab_size: usize,
}

impl Default for EditorGlobals {
    fn default() -> Self {
        Self {
            normal_mode_motion_tree: crate::motion_interpreter::MotionTree::default(),
            insert_mode_motion_tree: crate::motion_interpreter::MotionTree::default(),
            visual_mode_motion_tree: crate::motion_interpreter::MotionTree::default(),
            tab_size: 4,
        }
    }
}

pub(crate) struct EditorState {
    pub(crate) mode: EditorMode,
    pub(crate) cursor: crate::cursor::Cursor,
    pub(crate) command_line: CommandLine,
    pub(crate) buffers: Vec<crate::editor_buffer::Buffer>,
    pub(crate) buffer_idx: usize,
    pub(crate) term_info: TermInfo,
    pub(crate) motion_interpreter_state: crate::motion_interpreter::MotionInterpreterState,
    pub(crate) editor_globals: EditorGlobals,
}

pub(crate) enum EditorStateUpdate {
    None,
    Mode(EditorMode),
    Buffers(Vec<crate::editor_buffer::Buffer>),
    BufferIdx(usize),
    TermInfo(TermInfo),
    Full(EditorState),
}

impl EditorState {
    pub(crate) fn new_normal(term_info: TermInfo, editor_globals: EditorGlobals) -> Self {
        Self {
            mode: EditorMode::new_normal(),
            cursor: crate::cursor::Cursor::default(),
            command_line: CommandLine::default(),
            buffers: vec![crate::editor_buffer::Buffer::new()],
            buffer_idx: 0,
            term_info,
            motion_interpreter_state: crate::motion_interpreter::MotionInterpreterState::new(),
            editor_globals,
        }
    }

    pub(crate) fn new_insert(term_info: TermInfo, editor_globals: EditorGlobals) -> Self {
        Self {
            mode: EditorMode::new_insert(),
            cursor: crate::cursor::Cursor::default(),
            command_line: CommandLine::default(),
            buffers: vec![crate::editor_buffer::Buffer::new()],
            buffer_idx: 0,
            term_info,
            motion_interpreter_state: crate::motion_interpreter::MotionInterpreterState::new(),
            editor_globals,
        }
    }

    pub(crate) fn new_visual(
        cursor_start: crate::cursor::Cursor,
        term_info: TermInfo,
        editor_globals: EditorGlobals,
    ) -> Self {
        Self {
            mode: EditorMode::new_visual(cursor_start),
            cursor: crate::cursor::Cursor::default(),
            command_line: CommandLine::default(),
            buffers: vec![crate::editor_buffer::Buffer::new()],
            buffer_idx: 0,
            term_info,
            motion_interpreter_state: crate::motion_interpreter::MotionInterpreterState::new(),
            editor_globals,
        }
    }

    pub(crate) fn get_buffer(&self) -> &crate::editor_buffer::Buffer {
        &self.buffers[self.buffer_idx]
    }

    pub(crate) fn get_buffer_mut(&mut self) -> &mut crate::editor_buffer::Buffer {
        &mut self.buffers[self.buffer_idx]
    }

    pub(crate) fn active_motion_tree(&self) -> &crate::motion_interpreter::MotionTree {
        match self.mode {
            EditorMode::Normal => &self.editor_globals.normal_mode_motion_tree,
            EditorMode::Insert => &self.editor_globals.insert_mode_motion_tree,
            EditorMode::Visual { .. } => &self.editor_globals.visual_mode_motion_tree,
        }
    }

    pub(crate) fn active_motion_tree_mut(&mut self) -> &mut crate::motion_interpreter::MotionTree {
        match self.mode {
            EditorMode::Normal => &mut self.editor_globals.normal_mode_motion_tree,
            EditorMode::Insert => &mut self.editor_globals.insert_mode_motion_tree,
            EditorMode::Visual { .. } => &mut self.editor_globals.visual_mode_motion_tree,
        }
    }

    pub(crate) fn apply(&mut self, update: EditorStateUpdate) {
        match update {
            EditorStateUpdate::None => {}
            EditorStateUpdate::Mode(mode) => self.mode = mode,
            EditorStateUpdate::Buffers(buffers) => self.buffers = buffers,
            EditorStateUpdate::BufferIdx(buffer_idx) => self.buffer_idx = buffer_idx,
            EditorStateUpdate::TermInfo(term_info) => self.term_info = term_info,
            EditorStateUpdate::Full(new_state) => *self = new_state,
        }
    }
}
