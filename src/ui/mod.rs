pub mod theme;
pub mod header;
pub mod footer;
pub mod package_list;
pub mod detail;
pub mod dashboard;

use crate::app::App;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};

pub fn render(frame: &mut Frame, app: &mut App) {
    if app.show_dashboard {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // header
                Constraint::Min(0),     // dashboard
                Constraint::Length(1),  // footer
            ])
            .split(frame.area());

        header::render_header(frame, chunks[0], app);
        dashboard::render_dashboard(frame, chunks[1], app);
        footer::render_footer(frame, chunks[2], app);
    } else {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // header
                Constraint::Min(0),     // main content
                Constraint::Length(1),  // footer
            ])
            .split(frame.area());

        header::render_header(frame, chunks[0], app);

        // Split main content into list (left) and detail (right)
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(35),  // package list
                Constraint::Percentage(65),  // detail panel
            ])
            .split(chunks[1]);

        package_list::render_package_list(frame, main_chunks[0], app);
        detail::render_detail(frame, main_chunks[1], app);
        footer::render_footer(frame, chunks[2], app);
    }
}
