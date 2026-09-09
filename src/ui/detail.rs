use crate::app::{ActivePane, App, DetailTab};
use super::theme::get_theme;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub fn render_detail(frame: &mut Frame, area: Rect, app: &App) {
    let theme = get_theme();
    let is_active = app.active_pane == ActivePane::Detail;

    let border_style = if is_active {
        Style::default().fg(theme.border_active)
    } else {
        Style::default().fg(theme.border)
    };

    // Split area into tabs bar and content
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // tab bar
            Constraint::Min(0),    // content
        ])
        .split(area);

    // Render tab bar
    render_tab_bar(frame, chunks[0], app, &theme, border_style);

    // Render content based on active tab
    let pkg = match app.selected_package() {
        Some(p) => p,
        None => {
            let block = Block::default()
                .borders(Borders::ALL & !Borders::TOP)
                .border_style(border_style);
            let empty = Paragraph::new("No package selected")
                .block(block)
                .style(Style::default().fg(Color::DarkGray));
            frame.render_widget(empty, chunks[1]);
            return;
        }
    };

    let block = Block::default()
        .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        .border_style(border_style);

    let lines: Vec<Line> = match app.detail_tab {
        DetailTab::Info => build_info_lines(pkg, &theme),
        DetailTab::Files => build_files_lines(app, &theme),
        DetailTab::Dependencies => build_deps_lines(pkg, &theme),
    };

    let text = Text::from(lines);
    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((app.detail_scroll, 0));

    frame.render_widget(paragraph, chunks[1]);
}

fn render_tab_bar(
    frame: &mut Frame,
    area: Rect,
    app: &App,
    _theme: &super::theme::Theme,
    border_style: Style,
) {
    let tabs = [DetailTab::Info, DetailTab::Files, DetailTab::Dependencies];
    let mut spans = vec![Span::raw(" ")];

    for tab in &tabs {
        let label = format!(" {} ", tab.label());
        if *tab == app.detail_tab {
            spans.push(Span::styled(
                label,
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::styled(
                label,
                Style::default().fg(Color::DarkGray),
            ));
        }
        spans.push(Span::raw(" "));
    }

    // Add ←→ hint if detail pane is active
    if app.active_pane == ActivePane::Detail {
        spans.push(Span::styled(
            " ← → switch tabs",
            Style::default().fg(Color::DarkGray),
        ));
    }

    let block = Block::default()
        .borders(Borders::ALL & !Borders::BOTTOM)
        .border_style(border_style)
        .title(" Detail ");

    let line = Line::from(spans);
    let tabs_widget = Paragraph::new(line).block(block);
    frame.render_widget(tabs_widget, area);
}

fn build_info_lines<'a>(
    pkg: &'a crate::package::Package,
    theme: &super::theme::Theme,
) -> Vec<Line<'a>> {
    let mut lines = Vec::new();

    let add_field = |lines: &mut Vec<Line<'a>>, label: &'a str, value: &'a str| {
        lines.push(Line::from(vec![
            Span::styled(format!("{:<16}", label), theme.label),
            Span::styled(value, theme.value),
        ]));
    };

    lines.push(Line::from(vec![
        Span::styled(
            &pkg.name,
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(&pkg.version, Style::default().fg(Color::White)),
    ]));
    lines.push(Line::from(Span::styled(
        &pkg.description,
        Style::default().fg(Color::White),
    )));
    lines.push(Line::raw(""));

    add_field(&mut lines, "Install Date", &pkg.install_date);
    let reason_str = match pkg.install_reason {
        crate::package::InstallReason::Explicit => "Explicitly installed",
        crate::package::InstallReason::Dependency => "Installed as dependency",
        crate::package::InstallReason::Unknown => "Unknown",
    };
    add_field(&mut lines, "Reason", reason_str);

    let source_str = match pkg.source {
        crate::package::PackageSource::Official => "Official",
        crate::package::PackageSource::AUR => "AUR",
    };
    add_field(&mut lines, "Source", source_str);
    add_field(&mut lines, "Size", &pkg.installed_size);
    add_field(&mut lines, "Architecture", &pkg.architecture);

    if !pkg.url.is_empty() {
        add_field(&mut lines, "URL", &pkg.url);
    }

    if !pkg.licenses.is_empty() {
        let licenses_joined = pkg.licenses.join(", ");
        lines.push(Line::from(vec![
            Span::styled(format!("{:<16}", "Licenses"), theme.label),
            Span::styled(licenses_joined, theme.value),
        ]));
    }

    if !pkg.groups.is_empty() {
        let groups_joined = pkg.groups.join(", ");
        lines.push(Line::from(vec![
            Span::styled(format!("{:<16}", "Groups"), theme.label),
            Span::styled(groups_joined, theme.value),
        ]));
    }

    lines.push(Line::raw(""));
    add_field(&mut lines, "Packager", &pkg.packager);
    add_field(&mut lines, "Build Date", &pkg.build_date);

    if pkg.is_orphan {
        lines.push(Line::raw(""));
        lines.push(Line::from(Span::styled(
            "⚠ This package is an orphan (no package depends on it)",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
    }

    lines
}

fn build_files_lines<'a>(app: &'a App, theme: &super::theme::Theme) -> Vec<Line<'a>> {
    let mut lines = Vec::new();

    match app.selected_files() {
        None => {
            lines.push(Line::from(Span::styled(
                "Press Enter to load file list",
                Style::default().fg(Color::DarkGray),
            )));
        }
        Some(groups) => {
            if groups.is_empty() {
                lines.push(Line::from(Span::styled(
                    "No files found",
                    Style::default().fg(Color::DarkGray),
                )));
            } else {
                for group in groups {
                    // Category header
                    let header = format!(
                        "▸ {} ({} files)",
                        group.category,
                        group.files.len()
                    );
                    lines.push(Line::from(Span::styled(header, theme.category_header)));

                    // File paths
                    for file in &group.files {
                        lines.push(Line::from(vec![
                            Span::raw("  "),
                            Span::styled(file.as_str(), theme.file_path),
                        ]));
                    }
                    lines.push(Line::raw(""));
                }
            }
        }
    }

    lines
}

fn build_deps_lines<'a>(
    pkg: &'a crate::package::Package,
    theme: &super::theme::Theme,
) -> Vec<Line<'a>> {
    let mut lines = Vec::new();

    let add_section = |lines: &mut Vec<Line<'a>>, title: &str, items: &'a [String]| {
        lines.push(Line::from(Span::styled(
            title.to_string(),
            theme.category_header,
        )));
        if items.is_empty() {
            lines.push(Line::from(Span::styled(
                "  None",
                Style::default().fg(Color::DarkGray),
            )));
        } else {
            for item in items {
                lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(item.as_str(), theme.value),
                ]));
            }
        }
        lines.push(Line::raw(""));
    };

    add_section(&mut lines, "▸ Depends On", &pkg.depends);
    add_section(&mut lines, "▸ Optional Dependencies", &pkg.optional_deps);
    add_section(&mut lines, "▸ Required By", &pkg.required_by);
    add_section(&mut lines, "▸ Optional For", &pkg.optional_for);
    add_section(&mut lines, "▸ Provides", &pkg.provides);
    add_section(&mut lines, "▸ Conflicts With", &pkg.conflicts);
    add_section(&mut lines, "▸ Replaces", &pkg.replaces);

    lines
}
