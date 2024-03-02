mod gap_buffer;
mod motion_interpreter;
mod editor_state;
mod editor_buffer;
mod editor;
mod cursor;
mod setup_motions;

fn main() {
    editor::run().unwrap();
}
