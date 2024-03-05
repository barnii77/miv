use std::io::{self, Write};
use crossterm::{
    execute, queue,
    style::{self, Stylize}, cursor, terminal
};

pub(crate) fn render(ed_state: &crate::editor_state::EditorState) -> io::Result<()> {
    let mut stdout = io::stdout();

    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    for y in 0..ed_state.term_info.rows {
        for x in 0..ed_state.term_info.cols {
            if true { // TODO
                // in this loop we are more efficient by not flushing the buffer.
                queue!(stdout, cursor::MoveTo(x,y), style::PrintStyledContent("a".white()))?;
            }
        }
    }
    stdout.flush()?;
    Ok(())
}