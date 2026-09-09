use crate::app::App;
use super::theme::get_theme;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::text::{Line, Span};
use ratatui::style::{Style, Color, Modifier};

pub fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let theme = get_theme();

    let key_style = Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD);
    let desc_style = theme.footer;

    let spans = if app.show_dashboard {
        vec![
            Span::styled(" d", key_style),
            Span::styled(" Back  ", desc_style),
            Span::styled("q", key_style),
            Span::styled(" Quit", desc_style),
        ]
    } else if app.is_searching {
        vec![
            Span::styled(" Type", key_style),
            Span::styled(" to search  ", desc_style),
            Span::styled("Enter", key_style),
            Span::styled(" Confirm  ", desc_style),
            Span::styled("Esc", key_style),
            Span::styled(" Cancel", desc_style),
        ]
    } else {
        vec![
            Span::styled(" ↑↓", key_style),
            Span::styled(" Navigate  ", desc_style),
            Span::styled("Tab", key_style),
            Span::styled(" Switch pane  ", desc_style),
            Span::styled("/", key_style),
            Span::styled(" Search  ", desc_style),
            Span::styled("1-7", key_style),
            Span::styled(" Filter  ", desc_style),
            Span::styled("Enter", key_style),
            Span::styled(" Load files  ", desc_style),
            Span::styled("d", key_style),
            Span::styled(" Dashboard  ", desc_style),
            Span::styled("q", key_style),
            Span::styled(" Quit", desc_style),
        ]
    };

    let line = Line::from(spans);
    let footer = Paragraph::new(line);
    frame.render_widget(footer, area);
}
