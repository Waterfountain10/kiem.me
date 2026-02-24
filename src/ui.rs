use crate::app::{App, Section};
use crate::components::{content::render_content, footer::render_footer, header::render_header};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    prelude::Modifier,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

fn render_nav(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let mustard = Color::Rgb(184, 134, 11);

    let items = vec![
        ("1 Home", Section::Home),
        ("2 Projects", Section::Projects),
        ("3 Experience", Section::Experience),
        ("4 Systems", Section::Systems),
        ("5 Contact", Section::Contact),
    ];

    let spans: Vec<Span> = items
        .into_iter()
        .flat_map(|(label, section)| {
            let style = if app.section == section {
                Style::default().fg(mustard).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };

            vec![Span::styled(label, style), Span::raw("   ")]
        })
        .collect();

    let nav = Paragraph::new(Line::from(spans));
    frame.render_widget(nav, area);
}

pub fn draw(frame: &mut Frame, app: &App) {
    let size = frame.size();

    let outer = &Block::default()
        .title(" kiem.me ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    frame.render_widget(outer, size);

    // IMPORTANT: inner drawing area
    let inner = outer.inner(size);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // nav
            Constraint::Length(6), // header
            Constraint::Min(0),    // body
            Constraint::Length(1), // footer
        ])
        .split(inner);

    render_nav(frame, layout[0], app);
    render_header(frame, layout[1]);
    render_content(frame, layout[2], app);
    render_footer(frame, layout[3]);
}
