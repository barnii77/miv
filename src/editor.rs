use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal,
};
use std::fmt::Write;
use std::rc::Rc;

pub(crate) fn process(
    event: event::Event,
    ed_state: &mut crate::editor_state::EditorState,
) -> Result<
    Option<crate::motion_interpreter::MotionFunction>,
    crate::motion_interpreter::MotionInterpreterError,
> {
    match event {
        Event::FocusGained => Ok(None),
        Event::FocusLost => Ok(None),
        Event::Key(key_event) => {
            let KeyEvent {
                code, modifiers, ..
            } = key_event;
            let motion_atom = crate::motion_interpreter::MotionAtom { code, modifiers };
            let ed_state_updated_or_error = ed_state
                .motion_interpreter_state
                .clone() // this might hit performance hard because of heap allocations in vec
                // (TODO: remove vec component and replace with slice?)
                .update(ed_state.active_motion_tree(), motion_atom);
            match ed_state_updated_or_error {
                Ok(crate::motion_interpreter::MotionInterpreterState::Pending(
                    motion_component_buffer,
                )) => {
                    ed_state.motion_interpreter_state =
                        crate::motion_interpreter::MotionInterpreterState::Pending(
                            motion_component_buffer,
                        );
                    Ok(None)
                }
                Ok(crate::motion_interpreter::MotionInterpreterState::Done(motion_function)) => {
                    ed_state.motion_interpreter_state =
                        crate::motion_interpreter::MotionInterpreterState::new();
                    Ok(Some(crate::motion_interpreter::MotionFunction(Rc::clone(
                        &motion_function.0,
                    ))))
                }
                Err(error) => {
                    ed_state.motion_interpreter_state =
                        crate::motion_interpreter::MotionInterpreterState::new();
                    Err(error)
                }
            }
        }
        Event::Mouse(_mouse_event) => Ok(None), // TODO (maybe we dont need no mouse xD!!)
        Event::Paste(string) => {
            let buffer = ed_state.get_buffer_mut();
            let char_slice = string.chars().collect::<Vec<char>>();
            buffer.content.insert(&char_slice);
            Ok(None)
        }
        Event::Resize(rows, cols) => {
            ed_state.term_info.rows = rows;
            ed_state.term_info.cols = cols;
            Ok(None)
        }
    }
}

pub(crate) fn run() -> std::io::Result<()> {
    // NOTE: enabling and disabling raw mode is handled by main (caller)
    // let stdout = io::stdout();
    let (cols, rows) = terminal::size()?;
    let term_info = crate::editor_state::TermInfo { rows, cols };
    let editor_globals = crate::editor_state::EditorGlobals::default();
    let mut ed_state = crate::editor_state::EditorState::new_normal(term_info, editor_globals);
    match (
        &mut ed_state.editor_globals.normal_mode_motion_tree,
        &mut ed_state.editor_globals.insert_mode_motion_tree,
        &mut ed_state.editor_globals.visual_mode_motion_tree,
    ) {
        (
            crate::motion_interpreter::MotionTree::Tree(ref mut normal_motion_tree),
            crate::motion_interpreter::MotionTree::Tree(ref mut insert_motion_tree),
            crate::motion_interpreter::MotionTree::Tree(ref mut visual_motion_tree),
        ) => crate::setup_motions::setup_motions(
            normal_motion_tree,
            insert_motion_tree,
            visual_motion_tree,
        ),
        _ => unreachable!(),
    }
    loop {
        while event::poll(std::time::Duration::ZERO)? {
            let evnt = event::read()?;
            let process_result = process(evnt.clone(), &mut ed_state);
            match process_result {
                Ok(Some(motion_function)) => {
                    let update = motion_function.0(&ed_state);
                    ed_state.apply(update);
                }
                Ok(None) => {}
                Err(error) => {
                    // panic only in debug mode, ignore in release mode
                    // if cfg!(debug_assertions) {
                    // panic!("Error processing event: {:?}", e);
                    // eprintln!("Error processing event: {:?}", error);
                    match ed_state.mode {
                        crate::editor_state::EditorMode::Normal => {
                            // In normal mode, this is considered an error
                            write!(
                                &mut ed_state.command_line.buffer as &mut dyn Write,
                                "{:?}",
                                error
                            )
                            .expect("Fatal: Could not write to command line buffer");
                        }
                        crate::editor_state::EditorMode::Insert => {
                            if let Event::Key(KeyEvent { code, .. }) = evnt {
                                // In insert mode, this is just an insert
                                // The user is just trying to type something
                                // This is not actually an error, it was false alarm
                                // We should write what the user typed into the current buffer
                                let tab_size = ed_state.editor_globals.tab_size;
                                let current_buffer = ed_state.get_buffer_mut();
                                if let KeyCode::Backspace = code {
                                    current_buffer.content.delete(1);
                                } else {
                                    let keys = match code {
                                        KeyCode::Char(c) => c.to_string(),
                                        KeyCode::Enter => "\n".to_string(),
                                        KeyCode::Tab => str::repeat(" ", tab_size),
                                        _ => "".to_string(),
                                    };
                                    current_buffer
                                        .content
                                        .insert(&keys.chars().collect::<Vec<_>>());
                                }
                            }
                        }
                        crate::editor_state::EditorMode::Visual { .. } => {
                            // In visual mode, this is considered an error
                            write!(
                                &mut ed_state.command_line.buffer as &mut dyn Write,
                                "{:?}",
                                error
                            )
                            .expect("Fatal: Could not write to command line buffer");
                        }
                    }
                    crate::render::render(&ed_state)?;
                }
            }
        }
    }
}
