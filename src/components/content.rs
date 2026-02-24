use crate::app::{App, Section};
use ratatui::{
    Frame,
    style::{Color, Style},
    widgets::Paragraph,
};

pub fn render_content(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let text = match app.section {
        Section::Home => "Embedded Systems Engineer\nShopify × Qualcomm",
        Section::Projects => "Projects section",
        Section::Experience => "Experience section",
        Section::Systems => "Systems section",
        Section::Contact => "Contact section",
    };

    let body = Paragraph::new(text).style(Style::default().fg(Color::Gray));

    frame.render_widget(body, area);
}
