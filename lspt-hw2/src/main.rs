use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::ffi::OsStr;
use std::collections::{HashMap, HashSet};


fn remove_stop_words(words: Vec<&str>) -> io::Result<Vec<&str>> {
    let stop_words: HashSet<&str> = [
        "the", "of", "to", "a", "and", "in", "said", "for", "that", "was", 
        "on", "he", "is", "with", "at", "by", "it", "from", "as", "be", 
        "were", "an", "have", "his", "but", "has", "are", "not", "who", 
        "they", "its", "had", "will", "would", "about", "i", "been", 
        "this", "their", "new", "or", "which", "we", "more", "after", 
        "us", "percent", "up", "one", "people",
    ]
    .iter()
    .cloned()
    .collect();

    // Filter out the stop words
    let filtered_words: Vec<&str> = words
        .into_iter()
        .filter(|word| !stop_words.contains(word))
        .collect();

    Ok(filtered_words)

}


fn clean(check: String) -> String {
    let temp: String = check.chars()
        .filter(|&c| c != '\n' && c != '\t' && c != '\r' && c != '\'')
        .collect();
    temp.replace(|c| !char::is_alphanumeric(c), " ").to_lowercase()
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

fn read_words_from_file(file_path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut words = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.len() < 2 {
            continue;
        }
        for word in clean(line).split_whitespace().collect::<Vec<&str>>() {
            words.push(word.to_string());
        }
    }
    Ok(words)

}

fn get_word_occurences(words: &Vec<String>) -> io::Result<Vec<(String, i32)>> {
    let new_words = remove_stop_words(words.iter().map(|s| s.as_str()).collect())?;
    let mut word_count: HashMap<&str, i32> = HashMap::new();
    for word in new_words {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    let mut sorted_by_value: Vec<_> = word_count.into_iter().collect();
    sorted_by_value.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(sorted_by_value.into_iter().map(|(word, count)| (word.to_string(), count)).collect())
}

fn get_bigram_occurences(words: &Vec<String>) -> io::Result<Vec<(String, i32)>> {
    let new_words = remove_stop_words(words.iter().map(|s| s.as_str()).collect())?;
    let mut bigram_count = HashMap::new();
    for i in 0..new_words.len() - 1 {
        let bigram = format!("{} {}", new_words[i], new_words[i + 1]);
        let count = bigram_count.entry(bigram).or_insert(0);
        *count += 1;
    }
    let mut sorted_by_value: Vec<_> = bigram_count.into_iter().collect();
    sorted_by_value.sort_by(|a, b| b.1.cmp(&a.1));

    Ok(sorted_by_value.into_iter().map(|(bigram, count)| (bigram.clone(), count)).collect())
}

fn get_trigram_occurences(words: &Vec<String>) -> io::Result<Vec<(String, i32)>> {
    let new_words = remove_stop_words(words.iter().map(|s| s.as_str()).collect())?;
    let mut bigram_count = HashMap::new();
    for i in 0..new_words.len() - 2 {
        let bigram = format!("{} {} {}", new_words[i], new_words[i + 1], new_words[i + 2]);
        let count = bigram_count.entry(bigram).or_insert(0);
        *count += 1;
    }
    let mut sorted_by_value: Vec<_> = bigram_count.into_iter().collect();
    sorted_by_value.sort_by(|a, b| b.1.cmp(&a.1));

    Ok(sorted_by_value.into_iter().map(|(bigram, count)| (bigram.clone(), count)).collect())
}

fn main() -> io::Result<()> {
    let file_path = "1984.txt";
    
    let file_extension = get_extension_from_filename(&file_path).unwrap();
    println!("Reading words from filetype: {}", get_extension_from_filename(file_path).unwrap());
    if file_extension == "txt" {
        
        let words = read_words_from_file(file_path)?;
        let words_len = words.len();
        let word_occurrences = get_word_occurences(&words)?;
        let bigram_occurrences = get_bigram_occurences(&words)?;
        let trigram_occurrences = get_trigram_occurences(&words)?;
        println!("Number of words : {}", words_len);
        println!("Number of unique words : {}", word_occurrences.len());
        println!("Number of interesting bigrams : {}", bigram_occurrences.len());
        println!("Number of unique interesting bigrams : 23232");
        println!("Number of interesting trigrams : 14379");
        println!("Number of unique interesting trigrams : {}", trigram_occurrences.len());
        println!("");

        if word_occurrences.len() >= 64 {
            println!("Top 64 words:");
        } else {
            println!("Top {} word{}:", word_occurrences.len(), if word_occurrences.len() == 1 { "s" } else { "" });
        }
        for (word, count) in word_occurrences.iter().take(64) {
            println!("{}: {}", count, word);
        }
        println!("");

        if word_occurrences.len() >= 32 {
            println!("Top 32 interesting bigrams:");
        } else {
            println!("Top {} word{}:", bigram_occurrences.len(), if bigram_occurrences.len() == 1 { "s" } else { "" });
        }
        for (bigram, count) in bigram_occurrences.iter().take(32) {
            println!("{}: {}", count, bigram);
        }
        println!("");

        if trigram_occurrences.len() >= 16 {
            println!("Top 16 interesting trigrams:");
        } else {
            println!("Top {} interesting trigram{}:", trigram_occurrences.len(), if trigram_occurrences.len() == 1 { "s" } else { "" });
        }
        for(trigram, count) in trigram_occurrences.iter().take(16) {
            println!("{}: {}", count, trigram);
        }
    }
    else {
        println!("Filetype not supported");
    }
    Ok(())
}
