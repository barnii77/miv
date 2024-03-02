use std::rc::Rc;

pub(crate) fn setup_motions(motion_tree_map: &mut crate::motion_interpreter::MotionTreeMap) {
    motion_tree_map.insert(
        crate::motion_interpreter::MotionAtom {
            code: crossterm::event::KeyCode::Char('c'),
            modifiers: crossterm::event::KeyModifiers::CONTROL,
        },
        crate::motion_interpreter::MotionTree::Atom(crate::motion_interpreter::MotionFunction(
            Rc::new(|_| {
                println!("bye bye");
                std::process::exit(0)
            }),
        )),
    );
    motion_tree_map.insert(
        crate::motion_interpreter::MotionAtom {
            code: crossterm::event::KeyCode::Char('q'),
            modifiers: crossterm::event::KeyModifiers::NONE,
        },
        crate::motion_interpreter::MotionTree::Atom(crate::motion_interpreter::MotionFunction(
            Rc::new(|_| {
                println!("bye bye");
                std::process::exit(0)
            }),
        )),
    );
}
