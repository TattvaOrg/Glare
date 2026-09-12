use ratatui::style::{Color, Modifier, Style};

#[allow(dead_code)]
pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub highlight: Color,
    pub highlight_fg: Color,
    pub border: Color,
    pub border_active: Color,
    pub header_bg: Color,
    pub header_fg: Color,
    pub filter_active: Style,
    pub filter_inactive: Style,
    pub pkg_name: Style,
    pub pkg_version: Style,
    pub pkg_aur: Style,
    pub pkg_orphan: Style,
    pub label: Style,
    pub value: Style,
    pub search: Style,
    pub category_header: Style,
    pub file_path: Style,
    pub footer: Style,
    pub dashboard_title: Style,
    pub dashboard_stat: Style,
    pub bar_filled: Style,
    pub bar_empty: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: Color::Reset,
            fg: Color::Reset,
            highlight: Color::Cyan,
            highlight_fg: Color::Black,
            border: Color::DarkGray,
            border_active: Color::Cyan,
            header_bg: Color::Reset,
            header_fg: Color::White,
            filter_active: Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            filter_inactive: Style::default().fg(Color::DarkGray),
            pkg_name: Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
            pkg_version: Style::default().fg(Color::DarkGray),
            pkg_aur: Style::default().fg(Color::Magenta),
            pkg_orphan: Style::default().fg(Color::Yellow),
            label: Style::default().fg(Color::DarkGray),
            value: Style::default().fg(Color::White),
            search: Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
            category_header: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            file_path: Style::default().fg(Color::White),
            footer: Style::default().fg(Color::DarkGray),
            dashboard_title: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            dashboard_stat: Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
            bar_filled: Style::default().fg(Color::Cyan),
            bar_empty: Style::default().fg(Color::DarkGray),
        }
    }
}

pub fn get_theme() -> Theme {
    Theme::default()
}
