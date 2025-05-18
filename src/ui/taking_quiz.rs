use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Text,
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::{App, CurrentInputMode};
use crate::questions::Question::{MultipleChoices, TrueFalse};

pub fn draw_taking_quiz(f: &mut Frame, app: &App) {
    let size = f.area();
    f.render_widget(ratatui::widgets::Clear, size);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(15),
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(15),
            Constraint::Length(3), // Input area
        ])
        .split(size);
    let question_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::LightCyan));
    if let Some(current_question) = app.current_questions() {
        let question_text = Paragraph::new(Text::styled(
            current_question.get_question_text(),
            Style::default().fg(Color::White),
        ))
        .wrap(Wrap { trim: false })
        .block(question_block)
        .alignment(Alignment::Center);
        f.render_widget(question_text, chunks[0]);

        match current_question {
            MultipleChoices(mcq) => {
                let options_text = mcq
                    .options
                    .iter()
                    .enumerate()
                    .map(|(i, option)| {
                        let current_idx = app.current_question_index;
                        let style = if app.show_feedback == Some(true) {
                            if let Some(answer_index) = app.user_answer_mc.get(&current_idx) {
                                if i == mcq.correct_answer {
                                    Style::default()
                                        .fg(Color::Green)
                                        .add_modifier(Modifier::BOLD)
                                } else if *answer_index == i {
                                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                                } else {
                                    Style::default().fg(Color::White)
                                }
                            } else {
                                Style::default().fg(Color::White)
                            }
                        } else {
                            Style::default().fg(Color::White)
                        };
                        ListItem::new(format!("{}. {}", (i as u8 + b'A') as char, option))
                            .style(style)
                    })
                    .collect::<Vec<_>>();
                let option_list = List::new(options_text)
                    .block(Block::default().title("Options").borders(Borders::ALL));
                f.render_widget(option_list, chunks[3]);

                let input_area = Paragraph::new(app.answer_input.as_str()).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(match app.current_input_mode {
                            CurrentInputMode::Normal => "Press 'a' to answer",
                            CurrentInputMode::Answering => "Your Answer (A, B, C, D)",
                            _ => "",
                        })
                        .style(Style::default().fg(Color::Yellow)),
                );
                f.render_widget(input_area, chunks[4]);
            }
            TrueFalse(tfq) => {
                // Display True/False statements.  Now correctly using TrueFalseStatements
                let statements_text = tfq
                    .statements
                    .iter()
                    .enumerate()
                    .map(|(i, statement)| {
                        let current_idx = app.current_question_index;
                        let is_current = app.current_input_mode == CurrentInputMode::Answering
                            && i == app.current_tf_statement_index
                            && app.show_feedback.is_none();

                        let style = if is_current {
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD)
                        } else if app.show_feedback == Some(true) {
                            if let Some(answers) = app.user_answer_tf.get(&current_idx) {
                                if let Some(Some(answer)) = answers.get(i) {
                                    if *answer == statement.correct_answer {
                                        Style::default()
                                            .fg(Color::Green)
                                            .add_modifier(Modifier::BOLD)
                                    } else {
                                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                                    }
                                } else {
                                    Style::default().fg(Color::White)
                                }
                            } else {
                                Style::default().fg(Color::White)
                            }
                        } else {
                            Style::default().fg(Color::White)
                        };
                        ListItem::new(format!(
                            "{}. {} - {}",
                            i + 1,
                            statement.text,
                            if let Some(answers) = app.user_answer_tf.get(&current_idx) {
                                if let Some(Some(answer)) = answers.get(i) {
                                    if *answer { "True" } else { "False" }
                                } else {
                                    ""
                                }
                            } else {
                                ""
                            }
                        ))
                        .style(style)
                    })
                    .collect::<Vec<_>>();
                let statements_list = List::new(statements_text)
                    .block(Block::default().title("Statements").borders(Borders::ALL));
                f.render_widget(statements_list, chunks[3]);

                let input_area = Paragraph::new(app.answer_input.as_str()).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(match app.current_input_mode {
                            CurrentInputMode::Normal => "Press 'a' to answer",
                            CurrentInputMode::Answering => "Your Answer (T/F)",
                            _ => "",
                        })
                        .style(Style::default().fg(Color::Yellow)),
                );
                f.render_widget(input_area, chunks[4]);
            }
        }
    }
}
