use super::theme::get_theme;
use crate::app::App;
use crate::package::format_bytes;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;

pub fn render_dashboard(frame: &mut Frame, area: Rect, app: &App) {
    let theme = get_theme();

    let block = Block::default()
        .title(" Dashboard ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border_active));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(8), // stats
            Constraint::Min(0),    // top packages
        ])
        .split(block.inner(area));

    frame.render_widget(block, area);

    // === Stats section ===
    let total = app.total_packages();
    let explicit = app.explicit_count();
    let deps = app.dependency_count();
    let aur = app.aur_count();
    let orphans = app.orphan_count();
    let total_size = format_bytes(app.total_installed_size);

    let stats_lines = vec![
        Line::from(Span::styled("  Package Statistics", theme.dashboard_title)),
        Line::raw(""),
        Line::from(vec![
            Span::styled("    Total Packages:  ", theme.label),
            Span::styled(format!("{}", total), theme.dashboard_stat),
        ]),
        Line::from(vec![
            Span::styled("    ├── Explicit:    ", theme.label),
            Span::styled(format!("{}", explicit), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::styled("    ├── Dependencies:", theme.label),
            Span::styled(format!(" {}", deps), Style::default().fg(Color::Blue)),
        ]),
        Line::from(vec![
            Span::styled("    ├── AUR:         ", theme.label),
            Span::styled(format!("{}", aur), Style::default().fg(Color::Magenta)),
        ]),
        Line::from(vec![
            Span::styled("    └── Orphans:     ", theme.label),
            Span::styled(format!("{}", orphans), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("    Disk Usage:      ", theme.label),
            Span::styled(total_size, theme.dashboard_stat),
        ]),
    ];

    let stats = Paragraph::new(stats_lines);
    frame.render_widget(stats, chunks[0]);

    // === Top 10 Largest Packages ===
    let top = app.top_largest(10);
    let max_size = top.first().map(|p| p.installed_size_bytes).unwrap_or(1);

    let mut top_lines = vec![
        Line::from(Span::styled(
            "  Top 10 Largest Packages",
            theme.dashboard_title,
        )),
        Line::raw(""),
    ];

    for (i, pkg) in top.iter().enumerate() {
        let bar_width: usize = 30;
        let filled = if max_size > 0 {
            ((pkg.installed_size_bytes as f64 / max_size as f64) * bar_width as f64) as usize
        } else {
            0
        };
        let empty = bar_width.saturating_sub(filled);

        let bar_filled_str = "█".repeat(filled);
        let bar_empty_str = "░".repeat(empty);

        top_lines.push(Line::from(vec![
            Span::styled(format!("    {:<2} ", i + 1), theme.label),
            Span::styled(format!("{:<30}", pkg.name), theme.value),
            Span::styled(bar_filled_str, theme.bar_filled),
            Span::styled(bar_empty_str, theme.bar_empty),
            Span::raw(" "),
            Span::styled(format!("{:>10}", pkg.installed_size), theme.dashboard_stat),
        ]));
    }

    let top_widget = Paragraph::new(top_lines).wrap(Wrap { trim: false });
    frame.render_widget(top_widget, chunks[1]);
}
