use crossterm::execute;

mod cursor;
mod editor;
mod editor_buffer;
mod editor_state;
mod gap_buffer;
mod motion_interpreter;
mod setup_motions;
mod render;

// TODO refactor the crate to remove Cursor struct completely and instead compute it while
// rendering on the fly

pub(crate) fn panic(reason: &str) -> ! {
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        crossterm::cursor::MoveTo(0, 0)
    ).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();
    panic!("{}", reason)
}

pub(crate) fn quit() -> ! {
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        crossterm::cursor::MoveTo(0, 0)
    ).unwrap();
    crossterm::terminal::disable_raw_mode().unwrap();
    std::process::exit(0)
}

fn main() -> std::io::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    let error = editor::run();
    crossterm::terminal::disable_raw_mode()?;
    error
}
