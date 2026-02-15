use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("Alice".to_string(), 85);
    scores.insert("Bob".to_string(), 92);
    scores.insert("Charlie".to_string(), 78);

    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    let total: i32 = scores.values().sum();
    let count = scores.len() as f64;
    let average = total as f64 / count;
    println!("Average score: {:.1}", average);
}
