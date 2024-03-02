use crate::editor_state::{EditorState, EditorStateUpdate};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, thiserror::Error)]
pub(crate) enum MotionInterpreterError {
    #[error("Unknown motion: {0:?}")]
    UnknownMotionError(MotionComponentBuffer),
    // TODO when async execution is supported, the role of this
    // will change from meaning a previous command has not been
    // executed to meaning a previous command has not been scheduled
    // for execution yet
    #[error("Pending motion")]
    PendingMotionError,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum MotionTreeError {
    #[error("Empty motion")]
    EmptyMotionError,
    #[error("Motion already exists")]
    MotionAlreadyExistsError,
}

#[derive(Debug, Clone)]
pub(crate) struct MotionComponentBuffer(Vec<MotionAtom>);

// TODO use async; for example:
#[derive(Clone)]
pub(crate) struct MotionFunction(pub(crate) Rc<dyn Fn(&EditorState) -> EditorStateUpdate>);

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) struct MotionAtom {
    pub(crate) code: crossterm::event::KeyCode,
    pub(crate) modifiers: crossterm::event::KeyModifiers,
}

// so other modules can use it without having hardcoded copies
// this ideally avoids refactoring pain
pub(crate) type MotionTreeMap = HashMap<MotionAtom, MotionTree>;

pub(crate) enum MotionTree {
    Tree(MotionTreeMap),
    Atom(MotionFunction),
}

impl MotionTree {
    pub(crate) fn insert(
        &mut self,
        motion: MotionComponentBuffer,
        motion_function: MotionFunction,
    ) -> Result<(), MotionTreeError> {
        if motion.0.is_empty() {
            return Err(MotionTreeError::EmptyMotionError);
        }
        match self {
            Self::Tree(motion_tree) => {
                let mut possible_motions = motion_tree;
                // all motions except the last one are categorical (eg <leader>ds means <leader>
                // and d group it into category non-builtin and debugging)
                let categorical_motions_iter = motion.0.iter().take(motion.0.len() - 1);
                // NOTE: unwrap is fine because we return at function start if motion.is_empty()
                let final_motion = *motion.0.iter().last().unwrap();
                for m in categorical_motions_iter {
                    let subtree = possible_motions
                        .entry(*m)
                        .or_insert_with(MotionTree::default);
                    possible_motions = match subtree {
                        MotionTree::Tree(subtree) => subtree,
                        MotionTree::Atom(_) => {
                            return Err(MotionTreeError::MotionAlreadyExistsError)
                        }
                    };
                }
                possible_motions.insert(final_motion, MotionTree::Atom(motion_function));
                Ok(())
            }
            Self::Atom(_) => Err(MotionTreeError::EmptyMotionError),
        }
    }
}

impl Default for MotionTree {
    fn default() -> Self {
        Self::Tree(HashMap::new())
        // let mut hash_map = HashMap::new();
        // (Currently dont) automatically add ctrl-c to quit the editor
        // hash_map.insert(
        //     MotionAtom {
        //         code: crossterm::event::KeyCode::Char('c'),
        //         modifiers: crossterm::event::KeyModifiers::CONTROL,
        //     },
        //     Self::Atom(MotionFunction(Rc::new(|_| {
        //         println!("bye bye");
        //         std::process::exit(0)
        //     }))), // TODO improve this
        // );
        // MotionTree::Tree(hash_map)
    }
}

#[derive(Clone)]
pub(crate) enum MotionInterpreterState {
    Pending(MotionComponentBuffer),
    Done(MotionFunction),
}

impl MotionInterpreterState {
    pub(crate) fn new() -> Self {
        Self::Pending(MotionComponentBuffer(Vec::new()))
    }

    pub(crate) fn update(
        self,
        motion_tree: &MotionTree,
        next: MotionAtom,
    ) -> Result<Self, MotionInterpreterError> {
        match self {
            MotionInterpreterState::Pending(mut motion_component_buffer) => {
                let mut possible_motions = motion_tree;
                for m in motion_component_buffer.0.iter() {
                    match possible_motions {
                        MotionTree::Tree(motion_subtree) => {
                            let subtree = motion_subtree.get(m);
                            possible_motions = match subtree {
                                Some(subtree) => subtree,
                                None => {
                                    return Err(MotionInterpreterError::UnknownMotionError(
                                        motion_component_buffer,
                                    ))
                                }
                            }
                        }
                        MotionTree::Atom(motion_function) => {
                            return Ok(Self::Done(MotionFunction(Rc::clone(&motion_function.0))))
                        }
                    }
                }
                match possible_motions {
                    MotionTree::Tree(_) => {
                        motion_component_buffer.0.push(next);
                        Ok(Self::Pending(motion_component_buffer))
                    }
                    MotionTree::Atom(motion_function) => {
                        Ok(Self::Done(MotionFunction(Rc::clone(&motion_function.0))))
                    }
                }
            }
            MotionInterpreterState::Done(_) => Err(MotionInterpreterError::PendingMotionError),
        }
    }
}
