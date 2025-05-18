use crate::questions::{MultipleChoicesQuestion, Question, TrueFalseQuestion};
use rand::{rng, seq::SliceRandom};

pub fn shuffle_questions(questions: &mut Vec<Question>) {
    let mut rang = rng(); // Lấy bộ tạo số ngẫu nhiên
    questions.shuffle(&mut rang); // Xáo trộn Vec
}

pub fn shuffle_multichoices_question(mc_q: &mut MultipleChoicesQuestion) {
    let mut rang = rng();
    let correct_answer_text = mc_q.options[mc_q.correct_answer].clone();

    // Xáo trộn các lựa chọn
    mc_q.options.shuffle(&mut rang);

    // Tìm chỉ số mới của đáp án đúng
    if let Some(new_index) = mc_q
        .options
        .iter()
        .position(|opt| opt == &correct_answer_text)
    {
        mc_q.correct_answer = new_index;
    } else {
        eprintln!(
            "Error: Correct answer text not found after shuffling options for question: {}",
            mc_q.question_text
        );
    }
}

// Xáo trộn các phát biểu trong câu hỏi True/False
pub fn shuffle_truefalse_statements(tf_q: &mut TrueFalseQuestion) {
    let mut rang = rng();
    tf_q.statements.shuffle(&mut rang);
}

// Hàm tổng hợp để xáo trộn toàn bộ câu hỏi và các lựa chọn/phát biểu bên trong
pub fn prepare_questions_for_quiz(questions: &mut Vec<Question>) {
    // 1. Xáo trộn thứ tự các câu hỏi trước
    shuffle_questions(questions);

    // 2. Duyệt qua từng câu hỏi và xáo trộn các lựa chọn/phát biểu bên trong
    for question in questions.iter_mut() {
        match question {
            Question::MultipleChoices(mc_q) => {
                shuffle_multichoices_question(mc_q);
            }
            Question::TrueFalse(tf_q) => {
                shuffle_truefalse_statements(tf_q);
            }
        }
    }
}
