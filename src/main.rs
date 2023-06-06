pub mod line_parser;
pub mod process;
pub mod ui;

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread};
use tui::{backend::CrosstermBackend, Terminal};
use ui::list::draw_lists;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // ===============================

    // draw calls
    terminal.draw(draw_lists)?;

    // ===============================

    let event_thread = thread::spawn(|| loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: _,
            kind: KeyEventKind::Press,
            state: _,
        })) = event::read()
        {
            break;
        }
    });

    let _ = event_thread.join().unwrap();

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
