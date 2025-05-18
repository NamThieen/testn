use ratatui::{
    Frame,
    layout::Alignment,
    style::{Color, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::app::App;
use crate::ui::utils::centered_rect;

pub fn draw_exiting(f: &mut Frame, _app: &App) {
    let size = f.area();

    f.render_widget(ratatui::widgets::Clear, size);

    let popup_block = Block::default()
        .title("Confirm Exit")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::DarkGray).fg(Color::Red));

    let exit_text = Text::styled(
        "Are you sure you want to quit? (y/n)",
        Style::default().fg(Color::LightRed),
    );

    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    let area = centered_rect(60, 20, size); // Use 60% width, 20% height for the popup
    f.render_widget(exit_paragraph, area);
}
