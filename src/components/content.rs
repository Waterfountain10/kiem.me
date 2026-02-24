use crate::app::{App, Section, ViewMode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render_content(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let border_color = if app.scroll > 0 {
        Color::Rgb(184, 134, 11)
    } else {
        Color::DarkGray
    };
    match app.section {
        Section::Projects => render_projects(frame, area, app, border_color),
        Section::Home => render_home(frame, area, app, border_color),
        Section::Experience => render_experience(frame, area, app, border_color),
        Section::Systems => render_systems(frame, area, app, border_color),
        Section::Contact => render_contact(frame, area, app, border_color),
    }
}

pub fn render_experience(frame: &mut Frame, area: Rect, app: &App, border_color: Color) {
    let text = "
        OneSpan — BackEnd Developer Intern\n\
        \n\
        - SpringBoot, Terraform\n\
        - Cloud Microservices Team\n\
        Shopify — (Incoming Summer 2026) Infra Intern\n\
        \n\
        - Golang, Ruby\n\
        - Distributed Sytems\n\
        \n\
        Qualcomm — (Incoming Fall 2026) Compiler Toolchain Intern\n\
        \n\
        - C++, ARM_Assembly\n\
        - LLVM-based toolchains \n\";

    let block = Paragraph::new(text)
        .scroll((app.scroll, 0))
        .style(Style::default().fg(Color::Gray))
        .block(
            Block::default()
                .title(" Experience ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        );

    frame.render_widget(block, area);
}

fn render_systems(frame: &mut Frame, area: ratatui::layout::Rect, app: &App, border_color: Color) {
    let text = "Systems Focus\n\
         \n\
         - Cache invalidation strategies\n\
         - Memory layout awareness\n\
         - Instruction decoding & execution loops\n\
         - IR design and lowering pipelines\n\
         - Binary formats and parsing\n\
         - Toolchain architecture\n\
         \n\
         Philosophy\n\
         \n\
         Build small, composable, observable systems.\n";

    let block = Paragraph::new(text)
        .scroll((app.scroll, 0))
        .style(Style::default().fg(Color::Gray))
        .block(
            Block::default()
                .title(" Systems ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        );

    frame.render_widget(block, area);
}

fn render_contact(frame: &mut Frame, area: ratatui::layout::Rect, app: &App, border_color: Color) {
    let text = "GitHub\n\
         https://github.com/Waterfountain10\n\
         \n\
         LinkedIn\n\
         https://www.linkedin.com/in/william-k-lafond/\n\
         \n\
         Email\n\
         william.lafond@mail.mcgill.ca\n";

    let block = Paragraph::new(text)
        .scroll((app.scroll, 0))
        .style(Style::default().fg(Color::Gray))
        .block(
            Block::default()
                .title(" Contact ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        );

    frame.render_widget(block, area);
}

fn render_home(frame: &mut Frame, area: ratatui::layout::Rect, app: &App, border_color: Color) {
    let text = "Embedded Systems Engineer\n\
         \n\
         Incoming Shopify × Qualcomm\n\
         \n\
         Interests\n\
         - Compilers\n\
         - Emulators\n\
         - Distributed systems\n\
         - Infrastructure tooling\n";

    let block = Paragraph::new(text)
        .scroll((app.scroll, 0))
        .style(Style::default().fg(Color::Gray))
        .block(
            Block::default()
                .title(" Home ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color)),
        );

    frame.render_widget(block, area);
}

fn render_projects(frame: &mut Frame, area: ratatui::layout::Rect, app: &App, border_color: Color) {
    let mustard = Color::Rgb(184, 134, 11);

    if app.view_mode == ViewMode::ProjectDetail {
        render_project_detail(frame, area, app);
        return;
    }

    let split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    // LEFT PANEL (Project List)
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

    let list = List::new(items).block(
        Block::default()
            .title(" Projects ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color)),
    );

    frame.render_widget(list, split[0]);

    // RIGHT PANEL (Preview)
    let preview_text = match app.projects[app.project_index] {
        "Glazel" => "Distributed build cache\nGo · Redis · CLI",
        "GameDaddy" => "GameBoy emulator\nC++20 · SDL2",
        "DecentClang" => "Compiler pipeline\nRust · LLVMlite",
        _ => "",
    };

    let preview = Paragraph::new(preview_text)
        .style(Style::default().fg(Color::Gray))
        .block(
            Block::default()
                .title(" Preview ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

    frame.render_widget(preview, split[1]);
}

fn render_project_detail(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let mustard = Color::Rgb(184, 134, 11);

    let detail_text = match app.projects[app.project_index] {
        "Glazel" => {
            "Glazel\n\nDistributed build cache\n\n- Remote artifact store\n- Redis-backed metadata\n- CLI-driven invalidation\n\nPress 'b' to go back."
        }
        "GameDaddy" => {
            "GameDaddy\n\nGameBoy emulator\n\n- CPU cycle accuracy\n- Memory banking (MBC)\n- SDL2 rendering\n\nPress 'b' to go back."
        }
        "DecentClang" => {
            "DecentClang\n\nCompiler pipeline\n\n- Lexer\n- Parser\n- LLVMlite IR\n- x86 backend\n\nPress 'b' to go back."
        }
        _ => "",
    };

    let block = Paragraph::new(detail_text)
        .scroll((app.scroll, 0))
        .style(Style::default().fg(Color::Gray))
        .block(
            Block::default()
                .title(" Project Detail ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(mustard)),
        );

    frame.render_widget(block, area);
}
