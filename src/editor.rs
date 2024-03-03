use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{self, Stylize},
    terminal,
};
use std::io::{self, Write};
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
            // TODO add this to the motion tree automatically
            // if code == KeyCode::Char('c') && modifiers == KeyModifiers::CONTROL {
            //     // FIXME: ctrl-c for now will quit the editor
            //     std::process::exit(0);
            // }
        }
        Event::Mouse(mouse_event) => Ok(None), // TODO (maybe we dont need no mouse xD!!)
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
    match &mut ed_state.editor_globals.normal_mode_motion_tree {
        crate::motion_interpreter::MotionTree::Tree(hash_map) => {
            crate::setup_motions::setup_motions(hash_map)
        }
        crate::motion_interpreter::MotionTree::Atom(_) => unreachable!(),
    }
    loop {
        let e = event::read()?;
        let process_result = process(e, &mut ed_state);
        match process_result {
            Ok(Some(motion_function)) => {
                let update = motion_function.0(&ed_state);
                ed_state.apply(update);
            }
            Ok(None) => {}
            Err(error) => {
                // panic only in debug mode, ignore in release mode
                if cfg!(debug_assertions) {
                    // panic!("Error processing event: {:?}", e);
                    eprintln!("Error processing event: {:?}", error);
                }
            }
        }
    }
}
