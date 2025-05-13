use crate::question_loader::load_questions;
use crate::questions::Question;
use crate::shuffler::prepare_questions_for_quiz;
use crate::utils::get_user_input;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let questions_file_path_input = get_user_input("Enter the path to file: ")?;
    let questions_file_path = questions_file_path_input.trim();

    if questions_file_path.is_empty() {
        eprintln!("No file path entered. Please run the application again and provide a path.");
        return Err("No file path provided".into());
    }

    let mut questions = load_questions(questions_file_path)?;
    println!("\n--- Loaded Questions ---");

    prepare_questions_for_quiz(&mut questions);

    println!("\n--- Start test ---");
    let mut correct_points: f64 = 0.0;
    let mut total_points_possible: f64 = 0.0;

    for (i, question) in questions.into_iter().enumerate() {
        match question {
            Question::MultipleChoices(mcq) => {
                println!("\n--- Question #{}: (MultipleChoice) ---", i + 1);
                println!("Text: {}", mcq.question_text);
                for (idx, opt) in mcq.options.iter().enumerate() {
                    println!("{}. {}", (b'A' + idx as u8) as char, opt);
                }

                loop {
                    let user_answer = get_user_input("Answer: ")?.to_ascii_uppercase();

                    if user_answer.len() == 1 {
                        let char_code = user_answer.chars().next().unwrap(); // Safe due to len check
                        if char_code >= 'A' && char_code < (b'A' + mcq.options.len() as u8) as char
                        {
                            let chosen = (char_code as u8 - b'A') as usize;
                            if chosen == mcq.correct_answer_index {
                                correct_points += 1.0;
                                println!("Correct!");
                            } else {
                                println!("Incorrect!");
                            }
                            total_points_possible += 1.0;
                            break;
                        }
                    }
                    println!("Please enter a valid option (e.g., A, B, C) and try again!");
                }
            }

            Question::TrueFalse(tf) => {
                println!("\n--- Question #{}: (True/False Statements) ---", i + 1);
                println!("Text: {}", tf.question_text);

                let mut question_correct_statements = 0;

                // Loop through each statement within the True/False question
                for (idx, statement) in tf.statements.iter().enumerate() {
                    println!(" {}. {}", idx + 1, statement.text);

                    loop {
                        print!("  Statement {} (T/F): ", idx + 1); // Indent for clarity
                        let answer = get_user_input("")?.to_ascii_uppercase();
                        if answer == "T" {
                            if statement.correct {
                                question_correct_statements += 1; // Only increment if correct
                                println!("  Correct!");
                            } else {
                                println!("  Incorrect. This statement was False.");
                            }
                            break; // Exit loop for this statement
                        } else if answer == "F" {
                            if !statement.correct {
                                question_correct_statements += 1; // Only increment if correct
                                println!("  Correct!");
                            } else {
                                println!("  Incorrect. This statement was True.");
                            }
                            break; // Exit loop for this statement
                        }
                        println!("  Invalid input. Please enter T or F."); // Indent for clarity
                    }
                }

                // Define these constants for clarity and easy modification
                const TF_POINTS_1_CORRECT: f64 = 0.1;
                const TF_POINTS_2_CORRECT: f64 = 0.25;
                const TF_POINTS_3_CORRECT: f64 = 0.5;
                const TF_POINTS_4_CORRECT: f64 = 1.0;

                let points_for_tf_question = match question_correct_statements {
                    1 => TF_POINTS_1_CORRECT,
                    2 => TF_POINTS_2_CORRECT,
                    3 => TF_POINTS_3_CORRECT,
                    4 => TF_POINTS_4_CORRECT,
                    _ => 0.0,
                };

                correct_points += points_for_tf_question;
                total_points_possible += 1.0;

                println!(
                    "  You got {} out of {} statements correct for this True/False question, earning {:.2} points.",
                    question_correct_statements,
                    tf.statements.len(),
                    points_for_tf_question
                );
            }
        }
    }

    println!("\n--- Quiz Complete ---");
    println!(
        "You answered {:.2} out of {:.2} total points correctly.",
        correct_points, total_points_possible
    );

    let final_score = if total_points_possible > 0.0 {
        (correct_points / total_points_possible) * 10.0
    } else {
        0.0
    };

    println!("Your final score: {:.2}/10.0", final_score); // Changed "Your score" to "Your final score"
    Ok(())
}
