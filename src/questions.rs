use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MultipleChoicesQuestion {
    pub question_text: String,
    pub options: Vec<String>, // A, B, C, D choices
    pub correct_answer_index: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrueFalseStatements {
    pub text: String,
    pub correct: bool,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrueFalseQuestion {
    pub question_text: String,
    pub statements: Vec<TrueFalseStatements>, // True and False statements
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum Question {
    MultipleChoices(MultipleChoicesQuestion),
    TrueFalse(TrueFalseQuestion),
}
