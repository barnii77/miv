use crate::editor_state::MivState;
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
enum MotionInterpreterError {
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
enum MotionTreeError {
    #[error("Empty motion")]
    EmptyMotionError,
    #[error("Motion already exists")]
    MotionAlreadyExistsError,
}

#[derive(Debug)]
struct MotionComponentBuffer(Vec<char>);

// TODO use async; for example:
// type MotionFunction = fn(MivState) -> impl Future<Output=MivState>;
// type MotionFunction = fn(MivState) -> Box<dyn Future<Output=MivStateUpdate>>;
type MotionFunction = fn(MivState) -> MivState;

enum MotionTree {
    Tree(HashMap<char, MotionTree>),
    Atom(MotionFunction),
}

impl MotionTree {
    fn new() -> Self {
        Self::Tree(HashMap::new())
    }

    fn insert(
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
                    let subtree = possible_motions.entry(*m).or_insert_with(MotionTree::new);
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

enum MotionInterpreterState {
    Pending(MotionComponentBuffer),
    Done(MotionFunction),
}

impl MotionInterpreterState {
    fn update(self, motion_tree: MotionTree, next: char) -> Result<Self, MotionInterpreterError> {
        match self {
            MotionInterpreterState::Pending(mut motion_component_buffer) => {
                let mut possible_motions = &motion_tree;
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
                            return Ok(Self::Done(*motion_function));
                        }
                    }
                }
                match possible_motions {
                    MotionTree::Tree(_) => {
                        motion_component_buffer.0.push(next);
                        Ok(Self::Pending(motion_component_buffer))
                    }
                    MotionTree::Atom(motion_function) => Ok(Self::Done(*motion_function)),
                }
            }
            MotionInterpreterState::Done(_) => Err(MotionInterpreterError::PendingMotionError),
        }
    }
}
