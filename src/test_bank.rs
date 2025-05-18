use crate::questions::{MultipleChoicesQuestion, Question, TrueFalseQuestion, TrueFalseStatement};

pub fn get_sample_questions() -> Vec<Question> {
    vec![
        Question::MultipleChoices(MultipleChoicesQuestion {
            question_text: "What is the capital of France?".to_string(),
            options: vec![
                "Berlin".to_string(),
                "Madrid".to_string(),
                "Paris".to_string(),
                "Rome".to_string(),
            ],
            correct_answer: 2,
        }),
        Question::TrueFalse(TrueFalseQuestion {
            question_text: "Are these statements true or false?".to_string(),
            statements: vec![
                TrueFalseStatement {
                    text: "The sun is a star.".to_string(),
                    correct_answer: true,
                },
                TrueFalseStatement {
                    text: "The Earth is flat.".to_string(),
                    correct_answer: false,
                },
                TrueFalseStatement {
                    text: "Water boils at 100 degrees Celsius at sea level.".to_string(),
                    correct_answer: true,
                },
            ],
        }),
        Question::MultipleChoices(MultipleChoicesQuestion {
            question_text: "Which of these is a programming language?".to_string(),
            options: vec![
                "Rust".to_string(),
                "HTML".to_string(),
                "CSS".to_string(),
                "JSON".to_string(),
            ],
            correct_answer: 0,
        }),
        Question::TrueFalse(TrueFalseQuestion {
            question_text: "Assess the truthfulness of these statements about Rust:".to_string(),
            statements: vec![
                TrueFalseStatement {
                    text: "Rust is a compiled language.".to_string(),
                    correct_answer: true,
                },
                TrueFalseStatement {
                    text: "Rust has a garbage collector.".to_string(),
                    correct_answer: false,
                },
                TrueFalseStatement {
                    text: "Rust is known for its memory safety guarantees.".to_string(),
                    correct_answer: true,
                },
                TrueFalseStatement {
                    text: "Rust is primarily used for front-end web development.".to_string(),
                    correct_answer: false,
                },
            ],
        }),
    ]
}
