mod question_loader;
mod questions;
mod shuffler;
//mod test_bank; // Uncomment if you plan to use this module

use question_loader::load_questions;
use questions::Question;
use shuffler::prepare_questions_for_quiz;
use std::io::{Write, stdin, stdout};
// No longer need `use std::env;` as we're not using command-line arguments

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut questions_file_path_input = String::new();

    // Prompt the user for the file path
    print!("Enter the path to the questions: ");
    stdout().flush()?; // Ensure the prompt is displayed immediately
    stdin().read_line(&mut questions_file_path_input)?;

    let questions_file_path = questions_file_path_input.trim();

    // Check if the user provided an empty path and use a default if desired,
    // or enforce a non-empty path. For now, we'll just use whatever they type.
    if questions_file_path.is_empty() {
        eprintln!("No file path entered. Please run the application again and provide a path.");
        return Err("No file path provided".into()); // Exit with an error
    }

    let mut questions = load_questions(questions_file_path)?;
    println!("\n--- Loaded Question ---");

    prepare_questions_for_quiz(&mut questions);

    println!("\n--- Start test---");
    let mut correct_points: f64 = 0.0;
    let mut total_points_possible: f64 = 0.0;

    for (i, question) in questions.into_iter().enumerate() {
        match question {
            Question::MultipleChoices(mcq) => {
                println!("\n---Question #{}: (MultipleChoices)", i + 1);
                println!("Text: {}", mcq.question_text);
                for (idx, opt) in mcq.options.iter().enumerate() {
                    println!("{}. {}", (b'A' + idx as u8) as char, opt);
                }

                let mut user_input = String::new();

                loop {
                    print!("Answer: ");
                    stdout().flush()?;
                    user_input.clear();
                    stdin().read_line(&mut user_input)?;

                    let user_answer = user_input.trim().to_ascii_uppercase();

                    if user_answer.len() == 1 {
                        let char_code = user_answer.chars().next().unwrap();
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
                println!("\n---Question #{}:(TrueFalse)", i + 1);
                println!("Text: {}", tf.question_text);

                let mut question_correct_statements = 0;

                for (idx, statement) in tf.statements.iter().enumerate() {
                    println!(" {}. {}", idx + 1, statement.text);

                    let mut user_answer = String::new();
                    let mut is_statement_correct = false;

                    loop {
                        print!("  Statement {} (T/F): ", idx + 1);
                        stdout().flush()?;
                        user_answer.clear();
                        stdin().read_line(&mut user_answer)?;
                        let trimmed_answer = user_answer.trim().to_ascii_uppercase();

                        if trimmed_answer == "T" {
                            if statement.correct {
                                is_statement_correct = true;
                                println!("  Correct!");
                            } else {
                                println!("  Incorrect. This statement was False.");
                            }
                            break;
                        } else if trimmed_answer == "F" {
                            if !statement.correct {
                                is_statement_correct = true;
                                println!("  Correct!");
                            } else {
                                println!("  Incorrect. This statement was True.");
                            }
                            break;
                        }
                        println!("  Invalid input. Please enter T or F.");
                    }
                    if is_statement_correct {
                        question_correct_statements += 1;
                    }
                }

                let points_for_tf_question = match question_correct_statements {
                    1 => 0.1,
                    2 => 0.25,
                    3 => 0.5,
                    4 => 1.0,
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

    println!("Your score: {:.2}/10.0", final_score);
    Ok(())
}
