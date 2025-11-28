use std::io;
use std::io::Write;
use std::fs::{OpenOptions, metadata};

fn main() {
    // Check if file already exists
    let file_exists = metadata("SMIS.txt").is_ok();

    // Open file for appending
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("SMIS.txt")
        .expect("Cannot open file");

    // If file is NEW (not existing), write headings once
    if !file_exists {
        file.write_all("PAU SMIS\n".as_bytes()).expect("Write failed");
        file.write_all("Student Name | Matric Number | Department | Level\n".as_bytes())
            .expect("Write failed");
    }

    // Vectors for storing data
    let mut student_name: Vec<String> = Vec::new();
    let mut matric_number: Vec<String> = Vec::new();
    let mut department: Vec<String> = Vec::new();
    let mut level: Vec<u32> = Vec::new();

    // === INPUT SECTION ===
    let mut input = String::new();

    println!("Student name:");
    io::stdin().read_line(&mut input).unwrap();
    student_name.push(input.trim().to_string());

    input.clear();
    println!("Matric number:");
    io::stdin().read_line(&mut input).unwrap();
    matric_number.push(input.trim().to_string());

    input.clear();
    println!("Department:");
    io::stdin().read_line(&mut input).unwrap();
    department.push(input.trim().to_string());

    input.clear();
    println!("Level:");
    io::stdin().read_line(&mut input).unwrap();
    let lvl: u32 = input.trim().parse().expect("Invalid number");
    level.push(lvl);

    // === WRITE ONLY STUDENT RECORD ===
    let line = format!(
        "{} | {} | {} | {}\n",
        student_name[0], matric_number[0], department[0], level[0]
    );

    file.write_all(line.as_bytes()).expect("Failed to write");

    println!("Record saved successfully!");
}
