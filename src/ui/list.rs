#![allow(dead_code)]
use tui::{
    backend::Backend,
    style::{Color, Modifier, Style},
    widgets::{List, ListItem, ListState},
    Frame,
};

use super::skeleton::get_skeleton;

pub struct LineItem {
    pub items: Vec<String>,
    pub state: ListState,
}

impl LineItem {
    pub fn new(items: Vec<String>) -> LineItem {
        LineItem {
            items,
            state: ListState::default(),
        }
    }

    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
        self.state = ListState::default();
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub fn draw_lists<'a, B: Backend>(f: &mut Frame<B>) {
    let (i, _) = get_skeleton(f);
    let mut events = LineItem::new(vec![String::from("Item 1"), String::from("Item 2")]);
    let items: Vec<ListItem> = events
        .items
        .iter()
        .map(|i| ListItem::new(i.as_ref()).style(Style::default().fg(Color::Red)))
        .collect();
    let list = List::new(items)
        .block(i.block)
        .highlight_symbol(">> ")
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .style(Style::default().fg(Color::White));
    f.render_stateful_widget(list, i.rect, &mut events.state);
    events.next();
}
