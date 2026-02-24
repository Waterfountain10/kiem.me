use crate::app::{App, Section};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph},
};

pub fn render_content(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    match app.section {
        Section::Projects => render_projects(frame, area, app),
        Section::Home => render_home(frame, area),
        Section::Experience => render_simple(frame, area, "Experience section"),
        Section::Systems => render_simple(frame, area, "Systems section"),
        Section::Contact => render_simple(frame, area, "Contact section"),
    }
}

fn render_projects(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let mustard = Color::Rgb(184, 134, 11);

    let split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    let items: Vec<ListItem> = app
        .projects
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let style = if i == app.project_index {
                Style::default().fg(mustard).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };

            ListItem::new(Line::from(Span::styled(*p, style)))
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, split[0]);

    let detail_text = match app.projects[app.project_index] {
        "Glazel" => "Distributed build cache\nGo · Redis · CLI\nCache invalidation",
        "GameDaddy" => "GameBoy emulator\nC++20 · SDL2\nMemory banking",
        "DecentClang" => "Compiler pipeline\nRust · LLVMlite\nIR lowering",
        _ => "",
    };

    let detail = Paragraph::new(detail_text).style(Style::default().fg(Color::Gray));

    frame.render_widget(detail, split[1]);
}

fn render_home(frame: &mut Frame, area: ratatui::layout::Rect) {
    let text = "Embedded Systems Engineer\nShopify × Qualcomm";
    let p = Paragraph::new(text).style(Style::default().fg(Color::Gray));
    frame.render_widget(p, area);
}

fn render_simple(frame: &mut Frame, area: ratatui::layout::Rect, text: &str) {
    let p = Paragraph::new(text).style(Style::default().fg(Color::Gray));
    frame.render_widget(p, area);
}
