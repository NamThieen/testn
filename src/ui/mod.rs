pub mod exiting;
pub mod main_menu;
pub mod path_input;
pub mod quizresults;
pub mod taking_quiz;
pub mod utils;
use crate::app::App;
use ratatui::Frame;

// This is the main ui function that delegates to the specific screen modules.
pub fn ui(f: &mut Frame, app: &App) {
    f.render_widget(ratatui::widgets::Clear, f.area());
    match app.current_screen {
        crate::app::CurrentScreen::MainMenu => {
            if app.current_input_mode == crate::app::CurrentInputMode::FilePathInput {
                path_input::draw_path_input(f, app);
            } else {
                main_menu::draw_main_menu(f, app);
            }
        }
        crate::app::CurrentScreen::TakingQuiz => taking_quiz::draw_taking_quiz(f, app),
        crate::app::CurrentScreen::Exiting => exiting::draw_exiting(f, app),
        crate::app::CurrentScreen::QuizResults => quizresults::draw_results(f, app),
        _ => {} // Add other screens as they are created.
    }
}
