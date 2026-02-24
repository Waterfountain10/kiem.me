use crate::app::App;
use crate::components::{content::render_content, footer::render_footer, header::render_header};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let size = frame.size();

    let outer = Block::default()
        .title(" kiem.me ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    frame.render_widget(outer, size);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(size);

    render_header(frame, layout[0]);
    render_content(frame, layout[1], app);
    render_footer(frame, layout[2]);
}
