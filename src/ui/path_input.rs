use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::app::{self, App};
use crate::ui::utils::centered_rect;

pub fn draw_path_input(f: &mut Frame, app: &App) {
    let size = f.area();

    f.render_widget(ratatui::widgets::Clear, size);

    let popup_block = Block::default()
        .title("Enter the path to the file")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::DarkGray).fg(Color::Red));

    let input_area = Paragraph::new(app.file_path_input.as_str())
        .block(popup_block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .style(Style::default().bg(Color::DarkGray).fg(Color::White));

    let area = centered_rect(60, 20, size); // Use 60% width, 20% height for the popup
    f.render_widget(input_area, area);
}
