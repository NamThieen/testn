use serde_yaml;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

use crate::questions::Question;

pub fn load_questions(file_path: &str) -> Result<Vec<Question>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let questions: Vec<Question> = serde_yaml::from_reader(reader).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to deserialize YAML data: {}", e),
        )
    })?;

    println!(
        "Successfully loaded {} questions from file",
        questions.len()
    );
    Ok(questions)
}
pub fn save_questions(questions: &[Question], file_path: &str) -> Result<(), io::Error> {
    let file = File::create(file_path)?;
    let writer = BufWriter::new(file);

    serde_yaml::to_writer(writer, questions).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to serialize questions to yaml: {}", e),
        )
    })?;

    println!(
        "Successfully saved {} questions to file: {}",
        questions.len(),
        file_path
    );
    Ok(())
}
