use std::collections::HashMap;

struct MivState {}

enum MotionInterpreterError {
    UnknownMotionError(String),
}

type MotionComponentBuffer = Vec<char>;

enum MotionTree {
    Incomplete(HashMap<char, MotionTree>),
    Complete(fn(MivState) -> MivState),
}

impl MotionTree {
    fn new() -> Self {
        Self::Incomplete(HashMap::new())
    }
}

enum MotionInterpreterState {
    Pending(MotionComponentBuffer),
    Done(fn(MivState) -> MivState),
}

impl MotionInterpreterState {
    fn update(self, motion_tree: MotionTree, next: char) -> Result<Self, MotionInterpreterError> {
        match self {
            MotionInterpreterState::Pending(mut motion_component_buffer) => {
                let mut possible_motions = &motion_tree;
                for m in motion_component_buffer.iter() {
                    match possible_motions {
                        MotionTree::Incomplete(motion_subtree) => {
                            let subtree = motion_subtree.get(m);
                            possible_motions = match subtree {
                                Some(subtree) => subtree,
                                None => {
                                    return Err(
                                        MotionInterpreterError::UnknownMotionError(
                                            format!("Unknown motion: {}", motion_component_buffer.iter().collect::<String>())
                                        )
                                    )
                                },
                            }
                        }
                        MotionTree::Complete(motion_function) => {
                            return Ok(Self::Done(*motion_function));
                        }
                    }
                }
                match possible_motions {
                    MotionTree::Incomplete(_) => {
                        motion_component_buffer.push(next);
                        Ok(Self::Pending(motion_component_buffer))
                    }
                    MotionTree::Complete(motion_function) => Ok(Self::Done(*motion_function))
                }
            },
            MotionInterpreterState::Done(_) => panic!("Did not refresh state after a motion was complete, editor should have processed motion command but didn't"),
        }
    }
}

