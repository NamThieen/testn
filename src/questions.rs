use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MultipleChoicesQuestion {
    pub question_text: String,
    pub options: Vec<String>, // A, B, C, D choices
    pub correct_answer: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrueFalseStatement {
    pub text: String,
    pub correct_answer: bool, // Changed from correct to correct_answer
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrueFalseQuestion {
    pub question_text: String,
    pub statements: Vec<TrueFalseStatement>, // True and False statements
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum Question {
    MultipleChoices(MultipleChoicesQuestion),
    TrueFalse(TrueFalseQuestion),
}

impl Question {
    pub fn get_question_text(&self) -> &str {
        match self {
            Question::MultipleChoices(mcq) => &mcq.question_text,
            Question::TrueFalse(tfq) => &tfq.question_text,
        }
    }
}
