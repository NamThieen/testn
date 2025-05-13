mod question_loader;
mod questions;
mod run_quiz;
mod shuffler;
mod test_creator;
mod utils;
//mod test_bank;
use run_quiz::run;
use test_creator::create;
use utils::get_user_input;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("\n--- Main Menu ---");
        println!("1. Take a test");
        println!("2. Create a new test");
        println!("3. Quit");
        let choice = get_user_input("Enter your choice")?;

        match choice.as_str() {
            "1" => {
                if let Err(e) = run() {
                    eprintln!("Error during quiz: {}", e)
                }
            }
            "2" => {
                if let Err(e) = create() {
                    eprintln!("Error during test creation: {}", e)
                }
            }
            "3" => {
                println!("Exiting app. Good bye");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter 1, 2 or 3");
            }
        }
    }
    Ok(())
}
