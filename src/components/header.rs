use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

pub fn render_header(frame: &mut Frame, area: ratatui::layout::Rect) {
    let mustard = Color::Rgb(184, 134, 11);

    let text = vec![
        Line::from(Span::styled(
            "    k i e m",
            Style::default().fg(mustard).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "    ────────",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from("    william kiem lafond"),
        Line::from(""),
        Line::from("    Embedded Systems · Compilers · Infra"),
    ];

    let paragraph = Paragraph::new(text).style(Style::default().fg(Color::Gray));

    frame.render_widget(paragraph, area);
}
