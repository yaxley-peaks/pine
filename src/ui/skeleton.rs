use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    Frame,
};

pub struct Skeleton<'a> {
    pub block: Block<'a>,
    pub rect: Rect,
}

pub fn get_skeleton<'a, B: Backend>(f: &mut Frame<B>) -> (Skeleton<'a>, Skeleton<'a>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let input_block = Block::default().title("Input").borders(Borders::ALL);
    let output_block = Block::default().title("Output").borders(Borders::ALL);

    (
        Skeleton {
            block: input_block,
            rect: chunks[0],
        },
        Skeleton {
            block: output_block,
            rect: chunks[1],
        },
    )
}
