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
use ui::{list::LineItem, skeleton::get_skeleton};

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

    let items_mutex = Arc::new(Mutex::new(LineItem::new(lines)));

    // draw calls
    let items = Arc::clone(&items_mutex);
    terminal.draw(move |f| {
        let line_items: Vec<ListItem> = items
            .lock()
            .unwrap()
            .items
            .iter()
            .map(|x| ListItem::new(x.to_owned()).style(Style::default().fg(Color::Blue)))
            .collect();
        let (_i, _) = get_skeleton(f);
        let list = List::new(line_items)
            .block(_i.block)
            .highlight_symbol(">> ");

        f.render_stateful_widget(list, _i.rect, &mut items.lock().unwrap().state)
    })?;

    // ===============================

    let items = items_mutex.clone();
    let event_thread = thread::spawn(move || {
        let mut items = items.lock().unwrap();
        loop {
            match event::read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: _,
                    kind: KeyEventKind::Press,
                    state: _,
                }) => break,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('j'),
                    modifiers: _,
                    kind: KeyEventKind::Press,
                    state: _,
                }) => items.next(),
                _ => {}
            }
        }
    });

    event_thread.join().unwrap();

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
