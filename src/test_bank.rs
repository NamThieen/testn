use crate::questions::{MultipleChoicesQuestion, Question, TrueFalseQuestion};
pub fn create_test_bank() -> Vec<Question> {
    let mut question = Vec::new();

    question.push(Question::MultipleChoices(MultipleChoicesQuestion {
        question_text: "Dau la thu do Phap".to_string(),
        options: vec![
            "Berlin".to_string(),
            "Paris".to_string(),
            "London".to_string(),
            "Rome".to_string(),
        ],
     correct_answer_index: 1,
    }));``

    question.push(Question::TrueFalse(TrueFalseQuestion {
        question_text: "Consider a right-angled triangle ABC, where the right angle is at A. The lengths of the sides AB, AC, and BC are a, b, and c respectively.".to_string(),
        statements: vec![
            ("a) According to the Pythagorean theorem, a² + b² = c².".to_string(), true),
            ("b) If a = 3 and b = 4, then c = 5.".to_string(), true),
            ("c) The area of the triangle can be calculated as (a * c) / 2.".to_string(), false), // Should be (a * b) / 2
            ("d) The sum of angles B and C is 90 degrees.".to_string(), true),
        ],        
}));

    question.push(Question::MultipleChoices(MultipleChoicesQuestion {
        question_text: "Which of the following is a programming language?".to_string(),
        options: vec![
            "HTML".to_string(),
            "CSS".to_string(),
            "Rust".to_string(),
            "JPEG".to_string(),
        ],
        correct_answer_index: 2,
    }));
    question
}
