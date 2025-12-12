use std::collections::HashMap;
use std::io::{self, Write};
use std::fs::File;

pub fn run() {
    let mut text = String::new();
    println!("Enter some text:");

    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read input");

    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word.to_lowercase()).or_insert(0);
        *count += 1;
    }

    let mut counts: Vec<(&String, &i32)> = word_count.iter().collect();
    counts.sort_by(|a, b| b.1.cmp(a.1));

    // Write to file instead of println!
    let mut file = File::create("word_count.txt").expect("Failed to create file");
    writeln!(file, "Word frequencies:").expect("Failed to write header");
    
    for (word, count) in counts {
        writeln!(file, "{}: {}", word, count).expect("Failed to write to file");
    }
    
    println!("Results saved to word_count.txt");
}

pub fn run1() {
    let mut text = String::new();
    println!("Enter some text:");

    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read input");

    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word.to_lowercase()).or_insert(0);
        *count += 1;
    }

    let mut counts: Vec<(&String, &i32)> = word_count.iter().collect();
    counts.sort_by(|a, b| b.1.cmp(a.1));

    // Create and open the file for writing
    let mut file = File::create("word_count.txt").expect("Failed to create file");
    
    writeln!(file, "Word frequencies:").expect("Failed to write header");
    println!("Word frequencies:");

    for (word, count) in counts {
        let line = format!("{}: {}", word, count);
        writeln!(file, "{}", line).expect("Failed to write to file");
        println!("{}", line);
    }
    
    println!("Results saved to word_count.txt");
}