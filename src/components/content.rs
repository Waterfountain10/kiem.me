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
    let mustard = Color::Rgb(184, 134, 11);

    let lines = vec![
        Line::from(vec![
            Span::styled(
                "OneSpan",
                Style::default().fg(mustard).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" — Backend Developer Intern"),
        ]),
        Line::from(""),
        Line::from("  • Spring Boot, Terraform"),
        Line::from("  • Cloud Microservices Team"),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Shopify",
                Style::default().fg(mustard).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" — Incoming Summer 2026 (Infrastructure Intern)"),
        ]),
        Line::from(""),
        Line::from("  • Golang, Ruby"),
        Line::from("  • Distributed Systems"),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Qualcomm",
                Style::default().fg(mustard).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" — Incoming Fall 2026 (Compiler Toolchain Intern)"),
        ]),
        Line::from(""),
        Line::from("  • C++, ARM Assembly"),
        Line::from("  • LLVM-based toolchains"),
    ];

    let paragraph = Paragraph::new(lines)
        .style(Style::default().fg(Color::Gray))
        .block(
            Block::default()
                .title(" Experience ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .scroll((app.scroll, 0));

    let block = paragraph
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
         code what you enjoy; not what is hype.\n";

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
    let mustard = Color::Rgb(184, 134, 11);

    // Keep this left-aligned (no leading spaces).
    // The `\` at the end of the first line avoids an initial blank line.
    const ART: &str = "\
⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣤⣴⣶⣶⣶⣶⣆⢰⣦⣤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢀⣠⣶⣿⣿⣿⣿⣿⡿⢿⣿⣿⠀⣿⣿⣿⣿⣶⣄⡀⠀⠀⠀⠀⠀
⠀⠀⠀⢀⣴⣿⣿⣿⣿⣿⣿⣿⠃⠀⣿⠿⠛⠀⠻⢿⣿⣿⣿⣿⣿⣦⠀⠀⠀⠀
⠀⠀⢠⣾⣿⣿⣿⣿⣿⣿⣿⡟⠀⠀⠃⠀⠀⠀⢠⣾⣿⣿⣿⣿⣿⣿⣷⡄⠀⠀
⠀⢠⣿⣿⣿⣿⣿⣿⣿⡟⠁⠀⠀⠀⠀⠀⠀⠀⢛⣿⣿⣿⣿⣿⣿⣿⣿⣿⡄⠀
⠀⣾⣿⣿⣿⣿⣿⠿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⢹⣿⣿⣿⣿⣿⣿⣿⣿⣷⠀
⢸⣿⣿⣿⣿⣛⡉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⡆⢠⡈⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇
⢸⣿⣿⣿⣿⣟⣉⣁⠀⠀⠀⠀⠀⠀⠀⠀⣻⡇⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇
⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠇⠀⠀⣠⣴⣿⣿⠈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠙⣿⣿⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⠀⢿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡀⠀⠀⠀⠀⠈⠁⠀⢨⣿⣿⣿⣿⣿⣿⣿⣿⡿
⠀⠈⢿⣿⣿⣿⣿⣿⠿⢿⣿⡇⠀⠀⠀⠀⠀⣤⠀⢸⣿⣿⣿⣿⣿⣿⣿⡿⠁
⠀⠀⠈⢿⣿⣿⣿⠁⣴⣾⡿⠁⠀⠀⠀⠀⠀⠘⡇⢸⣿⣿⣿⣿⣿⣿⡟⠁⠀
⠀⠀⠀⠀⠙⢿⣿⡀⠿⣿⡧⠀⠀⠀⠀⠀⠀⢠⡄⢸⣿⣿⣿⣿⡿⠋⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠙⠻⢶⣤⣤⣾⠀⠀⠀⠀⢠⣼⡇⢸⣿⡿⠟⠋⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠙⠃⠀⠀⠀⠠⠿⠟⠃⠈⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀
";

    // Convert the art into Lines to avoid weird wrapping/indent artifacts.
    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::from("hello ~ my name is william."));
    lines.push(Line::from(""));

    lines.extend(ART.lines().map(Line::from));

    lines.push(Line::from(""));

    lines.push(Line::from(vec![Span::raw(
        "i am a programmer based in montreal",
    )]));
    lines.push(Line::from(vec![Span::raw(
        "i dragonboat with a national-level team.",
    )]));
    lines.push(Line::from(vec![Span::raw(
        "i love to build anything that is cool (most likely including a TUI",
    )]));
    lines.push(Line::from(
        "i am big music freak! i love learning about music around the globe (and decades).",
    ));
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(
        "My Top 3 Albums",
        Style::default()
            .fg(Color::Gray)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(vec![
        Span::raw("  • Nujabes —  "),
        Span::styled(
            "Modal Soul",
            Style::default().fg(mustard).add_modifier(Modifier::BOLD),
        ),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  • David Tao — "),
        Span::styled(
            "陶喆 同名專輯",
            Style::default().fg(mustard).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" (the blue album)"),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  • John Mayer — "),
        Span::styled(
            "Continuum",
            Style::default().fg(mustard).add_modifier(Modifier::BOLD),
        ),
    ]));

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Interests",
        Style::default()
            .fg(Color::Gray)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from("  • film cameras"));
    lines.push(Line::from("  • old videogames"));
    lines.push(Line::from("  • compilers"));
    lines.push(Line::from("  • CLIs"));

    let block = Paragraph::new(lines)
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
    let (title, tech, logo): (&str, &str, &[&str]) = match app.projects[app.project_index] {
        "Glazel" => (
            "Distributed build cache",
            "Go · Redis · CLI",
            &[
                "        ____",
                "      /_____/|",
                "     /_____/ |",
                "    /_____/  |",
                "    |_____|  /",
                "    |_____| /",
                "    |_____|/",
            ],
        ),
        "GameDaddy" => (
            "GameBoy emulator",
            "C++20 · SDL2",
            &[
                "   .───────────.",
                "   |  [====]    |",
                "   |  (o)  (o)  |",
                "   '───────────'",
            ],
        ),
        "DecentClang" => (
            "Compiler pipeline",
            "Rust · LLVMlite",
            &[
                "   src  →  IR  →  ASM",
                "   ───      ──      ───",
                "    ░░      ▒▒      ███",
            ],
        ),
        _ => ("", "", &[]),
    };

    let mut lines: Vec<Line> = Vec::new();

    if !title.is_empty() {
        lines.push(Line::from(Span::styled(
            title,
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            tech,
            Style::default().fg(Color::DarkGray),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(""));

    for l in logo {
        lines.push(Line::from(Span::styled(
            *l,
            Style::default().fg(mustard).add_modifier(Modifier::BOLD),
        )));
    }

    let preview = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Preview ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .style(Style::default().fg(Color::Gray));

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
