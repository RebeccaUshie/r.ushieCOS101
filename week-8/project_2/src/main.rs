use std::fs::File;
use std::io::Write;

fn main() {
    // Defining the student details in arrays
    let student_names = ["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemon"];
    let matric_numbers = ["ACC10211111", "ECO10110101", "CSC10328828", "EEE11022020", "MEE10202001"];
    let departments = ["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let levels = [300, 100, 200, 400, 100];

    // To display student details
    println!("PAU SMIS");
    println!("Student Name | Matric Number | Department  | Level");
    for i in 0..student_names.len() {
        println!(
            "{} | {} | {} | {}",
            student_names[i], matric_numbers[i], departments[i], levels[i]
        );
    }

    // Save student details to a file students
    let file = File::create("students.txt");
    if file.is_err() {
        println!("Error creating file!");
        return;
    }
    let mut file = file.unwrap();

    if writeln!(file, "PAU SMIS").is_err() {
        println!("Error writing to file!");
        return;
    }
    if writeln!(file, "Student Name | Matric Number | Department  | Level").is_err() {
        println!("Error writing to file!");
        return;
    }

    for i in 0..student_names.len() {
        if writeln!(
            file,
            "{} | {} | {} | {}",
            student_names[i], matric_numbers[i], departments[i], levels[i]
        )
        .is_err()
        {
            println!("Error writing to file!");
            return;
        }
    }

    println!("\nStudent details saved to 'students.txt'.");
}
