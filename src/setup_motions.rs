use std::rc::Rc;

pub(crate) fn setup_normal_motions(motion_tree_map: &mut crate::motion_interpreter::MotionTreeMap) {
    motion_tree_map.insert(
        crate::motion_interpreter::MotionAtom {
            code: crossterm::event::KeyCode::Esc,
            modifiers: crossterm::event::KeyModifiers::NONE,
        },
        crate::motion_interpreter::MotionTree::Atom(crate::motion_interpreter::MotionFunction(
            Rc::new(|_| {
                // println!("bye bye");
                crate::quit()
            }),
        )),
    );
    motion_tree_map.insert(
        crate::motion_interpreter::MotionAtom {
            code: crossterm::event::KeyCode::Char('i'),
            modifiers: crossterm::event::KeyModifiers::NONE,
        },
        crate::motion_interpreter::MotionTree::Atom(crate::motion_interpreter::MotionFunction(
            Rc::new(|_| {
                // println!("insert mode");
                crate::editor_state::EditorStateUpdate::Mode(
                    crate::editor_state::EditorMode::Insert,
                )
            }),
        )),
    );
    motion_tree_map.insert(
        crate::motion_interpreter::MotionAtom {
            code: crossterm::event::KeyCode::Char('v'),
            modifiers: crossterm::event::KeyModifiers::NONE,
        },
        crate::motion_interpreter::MotionTree::Atom(crate::motion_interpreter::MotionFunction(
            Rc::new(|ed_state| {
                // println!("visual mode");
                crate::editor_state::EditorStateUpdate::Mode(
                    crate::editor_state::EditorMode::Visual {
                        cursor_start: ed_state.cursor,
                    }
                )
            }),
        )),
    );
}

pub(crate) fn setup_insert_motions(motion_tree_map: &mut crate::motion_interpreter::MotionTreeMap) {
    motion_tree_map.insert(
        crate::motion_interpreter::MotionAtom {
            code: crossterm::event::KeyCode::Esc,
            modifiers: crossterm::event::KeyModifiers::NONE,
        },
        crate::motion_interpreter::MotionTree::Atom(crate::motion_interpreter::MotionFunction(
            Rc::new(|_| {
                // println!("normal mode");
                crate::editor_state::EditorStateUpdate::Mode(
                    crate::editor_state::EditorMode::Normal,
                )
            }),
        )),
    );
}

pub(crate) fn setup_visual_motions(motion_tree_map: &mut crate::motion_interpreter::MotionTreeMap) {
    motion_tree_map.insert(
        crate::motion_interpreter::MotionAtom {
            code: crossterm::event::KeyCode::Char('v'),
            modifiers: crossterm::event::KeyModifiers::NONE,
        },
        crate::motion_interpreter::MotionTree::Atom(crate::motion_interpreter::MotionFunction(
            Rc::new(|_| {
                // println!("normal mode");
                crate::editor_state::EditorStateUpdate::Mode(
                    crate::editor_state::EditorMode::Normal,
                )
            }),
        )),
    );
    motion_tree_map.insert(
        crate::motion_interpreter::MotionAtom {
            code: crossterm::event::KeyCode::Esc,
            modifiers: crossterm::event::KeyModifiers::NONE,
        },
        crate::motion_interpreter::MotionTree::Atom(crate::motion_interpreter::MotionFunction(
            Rc::new(|_| {
                crate::editor_state::EditorStateUpdate::Mode(
                    crate::editor_state::EditorMode::Normal,
                )
            }),
        )),
    );
}

pub(crate) fn setup_motions(
    normal_motion_tree_map: &mut crate::motion_interpreter::MotionTreeMap,
    insert_motion_tree_map: &mut crate::motion_interpreter::MotionTreeMap,
    visual_motion_tree_map: &mut crate::motion_interpreter::MotionTreeMap,
) {
    setup_normal_motions(normal_motion_tree_map);
    setup_insert_motions(insert_motion_tree_map);
    setup_visual_motions(visual_motion_tree_map);
}
