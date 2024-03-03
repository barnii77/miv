mod cursor;
mod editor;
mod editor_buffer;
mod editor_state;
mod gap_buffer;
mod motion_interpreter;
mod setup_motions;

pub(crate) fn quit() -> ! {
    crossterm::terminal::disable_raw_mode().unwrap();
    std::process::exit(0)
}

fn main() -> std::io::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    let error = editor::run();
    crossterm::terminal::disable_raw_mode()?;
    error
}
