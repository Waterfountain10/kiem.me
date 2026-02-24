use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::Paragraph,
};

pub fn render_footer(frame: &mut Frame, area: ratatui::layout::Rect) {
    let footer =
        Paragraph::new("<1-5> navigate   <q> quit").style(Style::default().fg(Color::DarkGray));

    frame.render_widget(footer, area);
}
