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
            Constraint::Length(12),
            Constraint::Min(0),
            Constraint::Length(3),
            Constraint::Length(0), // Input area
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
                // Determine the height needed for each option
                let num_options = mcq.options.len();

                // Create constraints for each option block
                let option_constraints: Vec<Constraint> =
                    (0..num_options).map(|_| Constraint::Length(5)).collect();

                // Split the dynamic area (main_chunks[1]) into chunks for each option
                let options_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(option_constraints)
                    .split(chunks[1]); // Use the dynamic area for options

                for (i, option) in mcq.options.iter().enumerate() {
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

                    let option_text = format!("{}. {}", (i as u8 + b'A') as char, option);

                    let option_paragraph = Paragraph::new(Text::styled(option_text, style))
                        .block(
                            Block::default()
                                .border_type(BorderType::Rounded)
                                .borders(Borders::ALL)
                                .title(format!("Option {}", (i as u8 + b'A') as char)), // Title for each option block
                        )
                        .wrap(Wrap { trim: true }); // **THIS IS WHERE WRAPPING HAPPENS**

                    // Render each option into its dedicated chunk
                    if let Some(chunk) = options_chunks.get(i) {
                        f.render_widget(option_paragraph, *chunk);
                    }
                }

                // Input area for Multiple Choice
                let input_area = Paragraph::new(app.answer_input.as_str()).block(
                    Block::default()
                        .border_type(BorderType::Rounded)
                        .borders(Borders::ALL)
                        .title(match app.current_input_mode {
                            CurrentInputMode::Normal => "Press 'a' to answer",
                            CurrentInputMode::Answering => "Your Answer (A, B, C, D)",
                            _ => "",
                        })
                        .style(Style::default().fg(Color::Yellow)),
                );
                f.render_widget(input_area, chunks[2]); // Assuming main_chunks[2] is your input area
            }
            TrueFalse(tfq) => {
                // Determine the height needed for each statement
                let statement_height = 5; // Adjust as needed for your content
                let num_statements = tfq.statements.len();

                // Create constraints for each statement block
                let statement_constraints: Vec<Constraint> = (0..num_statements)
                    .map(|_| Constraint::Length(statement_height))
                    .collect();

                // Split the dynamic area (main_chunks[1]) into chunks for each statement
                let statements_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(statement_constraints)
                    .split(chunks[1]); // Use the dynamic area for statements

                for (i, statement) in tfq.statements.iter().enumerate() {
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

                    let user_answer_display =
                        if let Some(answers) = app.user_answer_tf.get(&current_idx) {
                            if let Some(Some(answer)) = answers.get(i) {
                                if *answer { "True" } else { "False" }
                            } else {
                                "" // No answer yet
                            }
                        } else {
                            "" // No answers for this question yet
                        };

                    let statement_text =
                        format!("{}. {} - {}", i + 1, statement.text, user_answer_display);

                    let statement_paragraph = Paragraph::new(Text::styled(statement_text, style))
                        .block(
                            Block::default()
                                .border_type(BorderType::Rounded)
                                .borders(Borders::ALL)
                                .title(format!("Statement {}", i + 1)), // Title for each statement block
                        )
                        .wrap(Wrap { trim: true }); // **THIS IS WHERE WRAPPING HAPPENS**

                    // Render each statement into its dedicated chunk
                    if let Some(chunk) = statements_chunks.get(i) {
                        f.render_widget(statement_paragraph, *chunk);
                    }
                }

                // Input area for True/False
                let input_area = Paragraph::new(app.answer_input.as_str()).block(
                    Block::default()
                        .border_type(BorderType::Rounded)
                        .borders(Borders::ALL)
                        .title(match app.current_input_mode {
                            CurrentInputMode::Normal => "Press 'a' to answer",
                            CurrentInputMode::Answering => "Your Answer (T/F)",
                            _ => "",
                        })
                        .style(Style::default().fg(Color::Yellow)),
                );
                f.render_widget(input_area, chunks[2]); // Assuming main_chunks[2] is your input area
            }
        }
    }
}
