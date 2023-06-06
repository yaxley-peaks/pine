use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};

pub fn draw_skeleton<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let input_block = Block::default()
        .title("Input")
        .borders(Borders::ALL)
        .title_alignment(tui::layout::Alignment::Center);

    let output_block = Block::default()
        .title("Output")
        .borders(Borders::ALL)
        .title_alignment(tui::layout::Alignment::Center);

    f.render_widget(input_block, chunks[0]);
    f.render_widget(output_block, chunks[1]);
}
