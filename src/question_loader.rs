use serde_yaml;
use std::fs::File;
use std::io::{self, BufReader}; // Added 'Read' trait here // Added 'BufRead' for read_until

use crate::questions::Question;

pub fn load_questions(file_path: &str) -> Result<Vec<Question>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file); // Made mutable
    let questions: Vec<Question> = serde_yaml::from_reader(reader).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Khong vao du lieu duoc {}:", e),
        )
    })?;

    println!(
        "Successfully loaded {} questions from file",
        questions.len()
    );
    Ok(questions)
}
