use crate::questions::Question;
use std::collections::HashMap;

#[derive(PartialEq, Eq)]
pub enum CurrentScreen {
    MainMenu,
    TakingQuiz,
    CreatingTest,
    QuizResults,
    Exiting,
}

#[derive(PartialEq, Eq)]
pub enum CurrentInputMode {
    Normal,
    Answering,
    FilePathInput,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub current_input_mode: CurrentInputMode,

    pub questions: Vec<Question>,
    pub current_question_index: usize,

    pub answer_input: String,

    pub user_answer_mc: HashMap<usize, usize>,

    pub user_answer_tf: HashMap<usize, Vec<Option<bool>>>, // Store answers for each statement in a TF question
    pub current_tf_statement_index: usize,
    // grading logic
    pub correct_points: f32,
    pub total_points_possible: f32,

    pub feedback_show_at: Option<std::time::Instant>,
    pub show_feedback: Option<bool>,
    pub selected_menu_item: usize,

    pub file_path_input: String, // for menu input
}

impl App {
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::MainMenu,
            current_input_mode: CurrentInputMode::Normal,

            questions: Vec::new(),
            current_question_index: 0,
            answer_input: String::new(),

            user_answer_mc: HashMap::new(),
            user_answer_tf: HashMap::new(),
            current_tf_statement_index: 0,
            // grading logic
            correct_points: 0.0,
            total_points_possible: 0.0,

            feedback_show_at: None,
            show_feedback: None,
            selected_menu_item: 0,

            file_path_input: String::new(),
        }
    }

    pub fn load_questions(&mut self, questions: Vec<Question>) {
        self.questions = questions;
        self.current_question_index = 0; //reset
        self.user_answer_mc.clear(); // Clear any previous answers
        self.user_answer_tf.clear();
        self.correct_points = 0.0;
        self.total_points_possible = 0.0;
        self.show_feedback = None;
    }

    pub fn current_questions(&self) -> Option<&Question> {
        self.questions.get(self.current_question_index)
    }

    pub fn next_question(&mut self) {
        if self.current_question_index < self.questions.len() - 1 {
            self.current_question_index += 1;
            self.current_input_mode = CurrentInputMode::Normal; // Reset input mode when moving to next question
        } else {
            self.calculate_score();
            self.current_screen = CurrentScreen::QuizResults;
        }
    }

    pub fn previous_question(&mut self) {
        if self.current_question_index > 0 {
            self.current_question_index -= 1;
            self.current_input_mode = CurrentInputMode::Normal;
        }
    }

    pub fn submit_answer_mc(&mut self, answer_index: usize) {
        if let Some(q) = self.current_questions() {
            if let Question::MultipleChoices(mcq) = q {
                self.user_answer_mc
                    .insert(self.current_question_index, answer_index);
            }
        }
    }

    pub fn submit_answer_tf(&mut self, statement_index: usize, answer: bool) {
        if let Some(q) = self.current_questions() {
            if let Question::TrueFalse(tfq) = q {
                // Ensure the vector for the current question index exists
                let statement_len = tfq.statements.len();
                self.user_answer_tf
                    .entry(self.current_question_index)
                    .or_insert_with(|| vec![None; statement_len]);

                // ... then access the vector and update the specific statement
                if let Some(statement_answer_slot) = self
                    .user_answer_tf
                    .get_mut(&self.current_question_index)
                    .unwrap()
                    .get_mut(statement_index)
                {
                    *statement_answer_slot = Some(answer); // Store the answer
                }
            }
        }
    }
    pub fn calculate_score(&mut self) {
        self.correct_points = 0.0;
        self.total_points_possible = 0.0;
        for (index, question) in self.questions.iter().enumerate() {
            match question {
                Question::MultipleChoices(mcq) => {
                    self.total_points_possible += 1 as f32;
                    if let Some(answer_index) = self.user_answer_mc.get(&index) {
                        if *answer_index == mcq.correct_answer {
                            self.correct_points += 1 as f32;
                        }
                    }
                }
                Question::TrueFalse(tfq) => {
                    for (statement_index, statement) in tfq.statements.iter().enumerate() {
                        self.total_points_possible += 1 as f32;
                        if let Some(answers) = self.user_answer_tf.get(&index) {
                            if let Some(Some(answer)) = answers.get(statement_index) {
                                if *answer == statement.correct_answer {
                                    self.correct_points += 1 as f32;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
