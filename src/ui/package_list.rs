use crate::app::{App, ActivePane};
use crate::package::PackageSource;
use super::theme::get_theme;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::text::{Line, Span};
use ratatui::style::{Style, Modifier};

pub fn render_package_list(frame: &mut Frame, area: Rect, app: &mut App) {
    let theme = get_theme();
    let is_active = app.active_pane == ActivePane::List;

    let border_style = if is_active {
        Style::default().fg(theme.border_active)
    } else {
        Style::default().fg(theme.border)
    };

    let block = Block::default()
        .title(" Packages ")
        .borders(Borders::ALL)
        .border_style(border_style);

    // Build list items
    let items: Vec<ListItem> = app
        .filtered_indices
        .iter()
        .map(|&idx| {
            let pkg = &app.packages[idx];
            let mut spans = vec![
                Span::styled(&pkg.name, theme.pkg_name),
                Span::raw(" "),
                Span::styled(&pkg.version, theme.pkg_version),
            ];

            // AUR badge
            if pkg.source == PackageSource::AUR {
                spans.push(Span::raw(" "));
                spans.push(Span::styled("AUR", theme.pkg_aur));
            }

            // Orphan indicator
            if pkg.is_orphan {
                spans.push(Span::raw(" "));
                spans.push(Span::styled("⚠", theme.pkg_orphan));
            }

            ListItem::new(Line::from(spans))
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            Style::default()
                .bg(theme.highlight)
                .fg(theme.highlight_fg)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▸ ");

    frame.render_stateful_widget(list, area, &mut app.list_state);
}
