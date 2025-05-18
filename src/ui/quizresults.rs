use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::app::{self, App};

pub fn draw_results(f: &mut Frame, app: &App) {
    let size = f.area();
    f.render_widget(ratatui::widgets::Clear, size);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(6),
            Constraint::Length(3), // Input area
        ])
        .split(size);
    let title_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan));

    let title_text = Paragraph::new(Text::styled(
        "Quiz Results",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Magenta),
    ))
    .block(title_block)
    .alignment(Alignment::Center)
    .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(title_text, chunks[0]);

    let final_score = if app.total_points_possible > 0.0 {
        (app.correct_points / app.total_points_possible) * 10.0
    } else {
        0.0
    };

    let result_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan));
    let result_text = Paragraph::new(Text::styled(
        format!(
            "You answer {0}/{1} total questions correctly.\n 
Your final score is {2:.2} out of 10.0",
            app.correct_points, app.total_points_possible, final_score
        ),
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Green),
    ))
    .block(result_block);
    f.render_widget(result_text, chunks[2]);

    let instructions_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Cyan));
    let instructions_text = Paragraph::new(Text::styled(
        format!("You have completed the quiz! Press 'Enter' to return to the main menu.",),
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Green),
    ))
    .block(instructions_block);
    f.render_widget(instructions_text, chunks[3]);
}
