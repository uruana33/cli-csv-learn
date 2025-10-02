use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Student {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Age")]
    pub age: u8,
    #[serde(rename = "Score")]
    pub score: f32,
}

fn read_csv(path: &str) -> Vec<Student> {
    let mut result = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_path(path)
        .unwrap();

    let mut students = Vec::new();
    for result in result.deserialize::<Student>() {
        let student = result.unwrap();
        students.push(student);
    }
    students
}

fn write_json(students: &Vec<Student>, path: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(students)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    write_json(&read_csv(input), output)?;
    Ok(())
}
