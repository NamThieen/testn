use std::io::{Write, stdin, stdout};

pub fn get_user_input(prompt: &str) -> Result<String, std::io::Error> {
    print!("{} ", prompt);
    stdout().flush()?;
    let mut user_input = String::new();
    stdin().read_line(&mut user_input)?;
    Ok(user_input.trim().to_string())
}
