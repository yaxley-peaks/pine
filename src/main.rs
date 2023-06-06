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
use std::{
    borrow::BorrowMut,
    fs, io,
    sync::{Arc, Mutex},
    thread,
};
use tui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    widgets::{List, ListItem},
    Terminal,
};
use ui::{
    list::{draw_lists, LineItem},
    skeleton::get_skeleton,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // ===============================

    let lines = fs::read_to_string("other.txt")?
        .lines()
        .map(ToOwned::to_owned)
        .collect();

    let mut items = LineItem::new(lines);

    // draw calls
    terminal.draw(|f| {
        let (i, _) = get_skeleton(f);
        let list_items: Vec<ListItem> = items
            .items
            .iter()
            .map(|i| ListItem::new(i.as_ref()).style(Style::default().fg(Color::Blue)))
            .collect();

        let list = List::new(list_items).block(i.block);

        f.render_stateful_widget(list, i.rect, &mut items.state);
    })?;

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
