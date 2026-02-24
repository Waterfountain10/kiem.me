// src/components/status.rs

use crate::app::{App, Section, ViewMode};
use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render_status(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let mustard = Color::Rgb(184, 134, 11);

    let section_label = match app.section {
        Section::Home => "Home",
        Section::Projects => "Projects",
        Section::Experience => "Experience",
        Section::Systems => "Systems",
        Section::Contact => "Contact",
    };

    let mut right = String::new();

    if app.section == Section::Projects {
        let name = app.projects.get(app.project_index).copied().unwrap_or("—");
        right = format!(
            "Project: {} ({}/{})",
            name,
            app.project_index + 1,
            app.projects.len()
        );
    } else if app.view_mode == ViewMode::ProjectDetail {
        right = "Mode: ProjectDetail".to_string();
    }

    let line = Line::from(vec![
        Span::styled(
            format!("[{}]", section_label),
            Style::default().fg(mustard).add_modifier(Modifier::BOLD),
        ),
        Span::raw("   "),
        Span::styled(right, Style::default().fg(Color::Gray)),
        Span::raw("   "),
        Span::styled(
            format!("Scroll: {}", app.scroll),
            Style::default().fg(Color::Gray),
        ),
    ]);

    let widget = Paragraph::new(line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    frame.render_widget(widget, area);
}
