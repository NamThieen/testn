mod app;
mod question_loader;
mod questions;
mod shuffler;
mod test_bank;
//mod test_creator;
mod ui;
use crate::ui::ui;
use app::{App, CurrentInputMode, CurrentScreen};
use ratatui::{
    Terminal,
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::{Backend, CrosstermBackend},
};
use std::io::stderr;
use test_bank::get_sample_questions;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stderr = stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    app.current_screen = CurrentScreen::MainMenu;
    //restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Err(e) = res {
        print!("{e:?}");
    }
    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    app.load_questions(get_sample_questions());

    app.current_screen = CurrentScreen::MainMenu;
    loop {
        terminal.draw(|f| ui(f, app))?;
        if app.show_feedback == Some(true) {
            if let Some(shown_at) = app.feedback_show_at {
                if shown_at.elapsed().as_secs() > 3 {
                    app.next_question();
                    app.show_feedback = None;
                    app.feedback_show_at = None;
                    app.current_tf_statement_index = 0;
                }
            }
        }
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    app.current_screen = CurrentScreen::Exiting;
                }
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    if app.current_screen == CurrentScreen::Exiting {
                        return Ok(());
                    }
                }
                KeyCode::Char('n') | KeyCode::Char('N') => {
                    if app.current_screen == CurrentScreen::Exiting {
                        app.current_screen = CurrentScreen::MainMenu;
                    }
                }
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    if app.current_screen == CurrentScreen::QuizResults {
                        app.current_screen = CurrentScreen::MainMenu;
                        app.current_question_index = 0;
                        app.answer_input.clear();
                        app.user_answer_mc.clear();
                        app.user_answer_tf.clear();
                        app.correct_points = 0.0;
                        app.total_points_possible = 0.0;
                        app.show_feedback = None;
                    }
                }
                _ => {}
            }
            match app.current_screen {
                CurrentScreen::MainMenu => match app.current_input_mode {
                    CurrentInputMode::FilePathInput => match key.code {
                        KeyCode::Enter => {
                            let path = app.file_path_input.trim();
                            if !path.is_empty() {
                                match crate::question_loader::load_questions(path) {
                                    Ok(questions) => {
                                        app.load_questions(questions);
                                        app.current_screen = CurrentScreen::TakingQuiz;
                                        app.current_input_mode = CurrentInputMode::Normal;
                                        app.file_path_input.clear();
                                    }
                                    Err(e) => {
                                        app.file_path_input = format!("Failed to load: {e}");
                                    }
                                }
                                app.file_path_input.clear();
                            }
                        }
                        KeyCode::Esc => {
                            app.current_input_mode = CurrentInputMode::Normal;
                            app.file_path_input.clear();
                        }
                        KeyCode::Backspace => {
                            app.file_path_input.pop();
                        }
                        KeyCode::Char('s') => {
                            app.current_screen = CurrentScreen::TakingQuiz;
                        }
                        KeyCode::Char(c) => {
                            app.file_path_input.push(c);
                        }
                        _ => {}
                    },
                    _ => match key.code {
                        KeyCode::Char('1') => {
                            app.current_screen = CurrentScreen::TakingQuiz;
                            app.load_questions(get_sample_questions());
                        }
                        KeyCode::Char('3') | KeyCode::Char('l') => {
                            app.current_input_mode = CurrentInputMode::FilePathInput;
                            app.file_path_input.clear();
                        }
                        _ => {}
                    },
                },
                CurrentScreen::TakingQuiz => match app.current_input_mode {
                    CurrentInputMode::FilePathInput => {}
                    CurrentInputMode::Normal => match key.code {
                        KeyCode::Char('a') => {
                            app.current_input_mode = CurrentInputMode::Answering;
                            app.answer_input.clear();
                        }
                        KeyCode::Enter => {
                            if app.show_feedback == Some(true) {
                                app.next_question();
                                app.show_feedback = None;
                                app.feedback_show_at = None;
                                app.current_tf_statement_index = 0;
                            }
                            if app.current_question_index >= app.questions.len() {
                                app.current_screen = CurrentScreen::QuizResults;
                            }
                        }
                        KeyCode::Char('n') => {
                            if app.show_feedback == Some(true) {
                                app.next_question();
                                app.show_feedback = None;
                                app.feedback_show_at = None;
                                app.current_tf_statement_index = 0;
                            }
                        }
                        KeyCode::Char('p') => {
                            app.previous_question();
                        }
                        _ => {}
                    },
                    CurrentInputMode::Answering => match key.code {
                        KeyCode::Enter => {
                            // For MCQ
                            if let Some(crate::questions::Question::MultipleChoices(_)) =
                                app.current_questions()
                            {
                                if let Some(first_char) = app.answer_input.chars().next() {
                                    if let Some(chosen_idx) =
                                        first_char.to_ascii_lowercase().to_digit(36)
                                    {
                                        if chosen_idx >= 10 && chosen_idx <= 13 {
                                            app.submit_answer_mc(chosen_idx as usize - 10);
                                            app.answer_input.clear();
                                            app.current_input_mode = CurrentInputMode::Normal;
                                            app.show_feedback = Some(true);
                                            app.feedback_show_at = Some(std::time::Instant::now());
                                        }
                                    }
                                }
                            }
                            // For TrueFalse
                            if let Some(crate::questions::Question::TrueFalse(tfq)) =
                                app.current_questions()
                            {
                                let tf_len = tfq.statements.len(); // Copy the length, drop the borrow
                                if let Some(input_char) = app.answer_input.chars().next() {
                                    let user_answer = match input_char.to_ascii_lowercase() {
                                        't' => Some(true),
                                        'f' => Some(false),
                                        _ => None,
                                    };
                                    if let Some(answer) = user_answer {
                                        let idx = app.current_tf_statement_index;
                                        app.submit_answer_tf(idx, answer);
                                        app.answer_input.clear();
                                        app.current_tf_statement_index += 1;
                                        if app.current_tf_statement_index >= tf_len {
                                            app.current_input_mode = CurrentInputMode::Normal;
                                            app.show_feedback = Some(true);
                                            app.feedback_show_at = Some(std::time::Instant::now());
                                        }
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            app.answer_input.pop();
                        }
                        KeyCode::Esc => {
                            app.answer_input.clear();
                            app.current_input_mode = CurrentInputMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            app.answer_input.push(c);
                        }
                        _ => {}
                    },
                },
                CurrentScreen::QuizResults => match key.code {
                    KeyCode::Enter => {
                        app.current_screen = CurrentScreen::MainMenu;
                        app.current_question_index = 0;
                        app.answer_input.clear();
                        app.user_answer_mc.clear();
                        app.user_answer_tf.clear();
                        app.correct_points = 0.0;
                        app.total_points_possible = 0.0;
                        app.show_feedback = None;
                    }
                    _ => {}
                },
                CurrentScreen::CreatingTest => match key.code {
                    KeyCode::Esc => {
                        app.current_screen = CurrentScreen::MainMenu;
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    _ => {}
                },
            }
        }
    }
}
