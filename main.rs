use std::collections::HashMap;
use std::io::{self, Write};

struct Student {
    name: String,
    score: i32,
}

impl Student {
    fn new(name: &str, score: i32) -> Self {
        Self {
            name: name.to_string(),
            score,
        }
    }

    fn grade(&self) -> &str {
        match self.score {
            90..=100 => "A",
            80..=89 => "B",
            70..=79 => "C",
            60..=69 => "D",
            _ => "F",
        }
    }
}

fn main() {
    let students = vec![
        Student::new("Alice", 85),
        Student::new("Bob", 92),
        Student::new("Charlie", 78),
        Student::new("Diana", 95),
        Student::new("Eve", 61),
    ];

    println!("=== Student Report ===\n");

    for student in &students {
        println!(
            "{:<10} Score: {:>3}  Grade: {}",
            student.name,
            student.score,
            student.grade()
        );
    }

    let total: i32 = students.iter().map(|s| s.score).sum();
    let count = students.len() as f64;
    let average = total as f64 / count;

    println!("\n--- Summary ---");
    println!("Total students: {}", students.len());
    println!("Average score:  {:.1}", average);
    println!("Highest:        {}", students.iter().map(|s| s.score).max().unwrap_or(0));
    println!("Lowest:         {}", students.iter().map(|s| s.score).min().unwrap_or(0));

    print!("\nPress Enter to exit...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}
