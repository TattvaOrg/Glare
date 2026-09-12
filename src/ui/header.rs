use super::theme::get_theme;
use crate::app::{App, FilterCategory};
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let theme = get_theme();

    let mut spans = vec![
        Span::styled(
            " GLARE ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
    ];

    // Filter tabs
    for cat in FilterCategory::all_categories() {
        let label = format!(" {}:{} ", cat.shortcut(), cat.label());
        if *cat == app.filter {
            spans.push(Span::styled(label, theme.filter_active));
        } else {
            spans.push(Span::styled(label, theme.filter_inactive));
        }
        spans.push(Span::raw(" "));
    }

    // Search indicator
    if app.is_searching {
        spans.push(Span::raw("  "));
        spans.push(Span::styled("/ ", theme.search));
        spans.push(Span::styled(&app.search_query, theme.search));
        spans.push(Span::styled("█", Style::default().fg(Color::Yellow))); // cursor
    } else if !app.search_query.is_empty() {
        spans.push(Span::raw("  "));
        spans.push(Span::styled(
            format!("filter: {}", app.search_query),
            Style::default().fg(Color::Yellow),
        ));
    }

    let line = Line::from(spans);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .title_bottom(format!(" {} packages ", app.filtered_indices.len()));

    let header = Paragraph::new(line).block(block);
    frame.render_widget(header, area);
}
