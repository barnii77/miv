use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize},
    terminal,
};
use std::io::{self, Write};

pub(crate) fn render(ed_state: &crate::editor_state::EditorState) -> io::Result<()> {
    let mut stdout = io::stdout();

    let buffer = ed_state.get_buffer();
    let mut buff_iter = buffer.content.iter();

    let top_y = std::cmp::max(
        0,
        ed_state.cursor.y as isize - ed_state.term_info.rows as isize / 2,
    ) as usize;
    let bottom_y = (ed_state.cursor.y + ed_state.term_info.rows as usize / 2)
        .checked_sub(ed_state.editor_globals.bottom_rows_skipped)
        .unwrap_or_else(|| crate::panic("bottom_y underflow: resized window too small to render"));

    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    let mut n_lines = 0;
    for &c in buff_iter.by_ref() {
        if n_lines == top_y {
            break;
        }
        if c == '\n' {
            n_lines += 1;
        }
    }

    let top_offset = std::cmp::max(
        0,
        ed_state.term_info.rows as isize / 2 - ed_state.cursor.y as isize,
    ) as usize;

    queue!(stdout, cursor::MoveToNextLine(top_offset as u16))?;
    // n_lines += top_offset;

    for &c in buff_iter.by_ref() {
        if c == '\n' {
            n_lines += 1;
            queue!(stdout, cursor::MoveToNextLine(1))?;
            if n_lines == bottom_y {
                break;
            }
        } else {
            queue!(stdout, style::PrintStyledContent(c.white()))?;
        }
    }
    queue!(
        stdout,
        cursor::MoveToNextLine(
            (bottom_y - n_lines) as u16 + ed_state.editor_globals.bottom_rows_skipped as u16
        )
    )?;
    for c in ed_state.command_line.buffer.chars() {
        if c != '\n' {
            queue!(stdout, style::PrintStyledContent(c.white()))?;
        }
    }
    queue!(stdout, cursor::MoveTo(ed_state.cursor.x as u16, ed_state.cursor.y as u16))?;

    // NOTE: no need to check if the bottom_y is out of range (cursor at bottom of file) because
    // the buff_iter.by_ref for loop writing the characters will simply stop iterating and write
    // nothing to the screen below the bottom_y. This is only needed for the top_y.
    stdout.flush()?;
    Ok(())
}
