




use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Open the file
    let file = File::open("ielts_feedback.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    
    // Create a HashMap to store word counts
    let mut word_counts = HashMap::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        
        // Split the line into words
        for word in line.split_whitespace() {
            // Convert to lowercase and remove non-alphabetic characters
            let cleaned_word = word.to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>();
            
            // Increment the count for this word
            *word_counts.entry(cleaned_word).or_insert(0) += 1;
        }
    }

    // Convert the HashMap to a vector of (word, count) pairs
    let mut word_vec: Vec<_> = word_counts.into_iter().collect();

    // Sort the vector by count in descending order
    word_vec.sort_by(|a, b| b.1.cmp(&a.1));

    // Print the 5 most common words
    println!("The 5 most common words are:");
    for (word, count) in word_vec.iter().take(5) {
        println!("{}: {}", word, count);
    }
}
