use crate::utils::get_user_input;
use crate::{
    question_loader::save_questions,
    questions::{
        MultipleChoicesQuestion,
        Question::{self, MultipleChoices, TrueFalse},
        TrueFalseQuestion, TrueFalseStatements,
    },
};

pub fn create() -> Result<(), Box<dyn std::error::Error>> {
    let mut new_questions: Vec<Question> = Vec::new();

    println!("\n--- Create New Test ---");
    let output_file_path = get_user_input("Enter the filename to save the test (vd: toan.yaml): ")?;

    if output_file_path.is_empty() {
        eprintln!("No output filename entered. Test creation cancelled.");
        return Err("No output file path provided ".into());
    }

    loop {
        println!("Do you want to add a question? (y/n)");
        let add_more = get_user_input("Choice: ")?.to_ascii_lowercase();

        if add_more == "n" {
            break;
        } else if add_more == "y" {
            println!("\n Select question type:");
            println!("  1. Multiple Choice (MC)");
            println!("  2. True/False Statements (TF)");
            let q_type = get_user_input("Enter choice (MC/TF): ")?.to_ascii_uppercase();

            match q_type.as_str() {
                "MC" => {
                    let question_text = get_user_input("Enter questions text: ")?;
                    let mut options: Vec<String> = Vec::new();
                    loop {
                        let option_text = get_user_input(&format!(
                            "  Enter options {} (leave empty for finish options): ",
                            options.len() + 1
                        ))?;
                        if option_text.is_empty() {
                            break;
                        }
                        options.push(option_text);
                    }

                    if options.is_empty() {
                        println!("Warning: no options provided. Skipping question");
                        continue;
                    }

                    let correct_answer_index: usize;
                    loop {
                        let prompt = format!("Enter correct answer index (1-{}): ", options.len());
                        let input_index_str = get_user_input(&prompt)?;

                        if let Ok(idx) = input_index_str.parse::<usize>() {
                            if idx > 0 && idx <= options.len() {
                                correct_answer_index = idx - 1;
                                break;
                            } else {
                                println!(
                                    "Invalid index. Please enter a number between 1 and {}",
                                    options.len()
                                )
                            }
                        }
                    }
                    new_questions.push(MultipleChoices(MultipleChoicesQuestion {
                        question_text,
                        options,
                        correct_answer_index,
                    }));
                }
                "TF" => {
                    let question_text =
                        get_user_input("Enter the main context for the question: ")?;
                    let mut statements: Vec<TrueFalseStatements> = Vec::new();

                    loop {
                        let statement_text = get_user_input(&format!(
                            "  Enter statements text {} leave empty to finish statements): ",
                            statements.len() + 1
                        ))?;
                        if statement_text.is_empty() {
                            break;
                        }
                        let correct: bool;
                        loop {
                            let tf_input = get_user_input("  Is this true or false? (T/F): ")?
                                .to_ascii_uppercase();
                            if tf_input == "T" {
                                correct = true;
                                break;
                            } else if tf_input == "F" {
                                correct = false;
                                break;
                            } else {
                                println!("Invalid input!")
                            }
                        }
                        statements.push(TrueFalseStatements {
                            text: statement_text,
                            correct,
                        });
                    }

                    if statements.is_empty() {
                        println!("Warning: No statements for this questions. Skipping questions");
                        continue;
                    }

                    new_questions.push(TrueFalse(TrueFalseQuestion {
                        question_text,
                        statements,
                    }));

                    println!("True/False statements added successfully!");
                }
                _ => {
                    println!("Invalid question type. Please enter 'MC' or 'TF'");
                }
            }
        } else {
            println!("Invalid choice!!");
        }
    }
    if new_questions.is_empty() {
        println!("No new questions was added. No file will be saved.");
    } else {
        save_questions(&new_questions, &output_file_path)?;
    }
    Ok(())
}
