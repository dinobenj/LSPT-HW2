use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::ffi::OsStr;
use std::collections::HashMap;
use std::env;

/**
 * Refactor 1: Extract Function (n-grams)
 * 
 * Refactor 2: Rename variable: 
 * 
 * Refactor 3: Replace Temp with Query: 
 * 
 * Refactor 4: 
 * 
 * Refactor 5: 
 * 
 * Refactor 6: 
 * 
 */

const STOP_WORDS: &'static[&'static str] = &[
    "the", "of", "to", "a", "and", "in", "said", "for", "that", "was", 
    "on", "he", "is", "with", "at", "by", "it", "from", "as", "be", 
    "were", "an", "have", "his", "but", "has", "are", "not", "who", 
    "they", "its", "had", "will", "would", "about", "i", "been", 
    "this", "their", "new", "or", "which", "we", "more", "after", 
    "us", "percent", "up", "one", "people",
];


/**
 * Sanitize a line of text and prepare it for further processing.
 * This function turns all non-alpha characters into whitespace except for the first apostrophe in a word.
 * All alpha characters are returned as alphabetic.
 */
fn clean(check: String) -> String {
    let mut temp: String = check.chars()
        .filter(|&c| c != '\n' && c != '\t' && c != '\r' && c != '«' && c != '»' && c != '×')
        .collect();

    temp = temp.replace(|c: char| !c.is_ascii(), " ");

    // parse entire line
    let mut apostrophe_count: i32 = 0;
    let mut in_word: bool = false;
    let mut last_apostrophe: bool = false;

    for (i, c) in temp.clone().char_indices() {
        if !char::is_alphabetic(c) { 
            if c == '\'' && apostrophe_count == 0 && in_word {
                // allow only one apostrophe 
                apostrophe_count += 1;
                last_apostrophe = true;
            } else {
                // turn all other non-alphabetical characters or additional apostrophes into whitespace
                if last_apostrophe {
                    temp.replace_range(i-1..i, " ");
                    last_apostrophe = false;
                }
                apostrophe_count = 0;
                in_word = false;
                temp.replace_range(i..i+1, " ");
            }
        } else {
            if last_apostrophe {
                last_apostrophe = false;
            }
            in_word = true;
        }
    }

    if last_apostrophe {
        temp.replace_range(temp.len()-1..temp.len(), " ");
    }


    temp.to_lowercase()
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

/**
 * Get a list of each word from a file.
 * This method returns words that are processed to be entirely in lowercase, with at most one apostrophe.
 * All other characters are treated as delimeters, and are filtered out.
 * 
 * For example, the sequence: "I'm...a word?" will yield the words ["i'm", "a", "word"].
 * 
 * Notably, stop words and short words are not filtered out.
 */
fn read_words_from_file(file_path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut words = Vec::new();

    // Process each word, line by line, then add to word list.
    for line in reader.lines() {
        let line = line?;
        for word in clean(line).split_whitespace().collect::<Vec<&str>>() {
            if word.len() == 0 {
                continue;
            }

            words.push(word.to_string());
        }
    }
    Ok(words)
}

fn get_ngram_occurrences(words: &Vec<String>, n: i32) -> io::Result<Vec<(String, i32)>> {
    let mut ngram_count = HashMap::new();

    if words.len() == 0 {
        return Ok(Vec::new());
    }

    for i in 0..words.len() - (n-1) as usize {
        let mut ngram = String::new();
        for j in 0..n {
            ngram.push_str(&words[i+j as usize]);
            ngram.push_str(" ");
        }
        ngram.pop();

        let mut bad: bool = false;
        for word in STOP_WORDS {
            for j in 0..n {
                if words[i+j as usize].len() < 2 || words[i+j as usize] == *word {
                    bad = true;
                    break;
                }
            }
        }
        if bad {
            continue;
        }
        let count = ngram_count.entry(ngram).or_insert(0);
        *count += 1;
    }

    Ok(ngram_count.into_iter().map(|(bigram, count)| (bigram.clone(), count)).collect())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut valid_documents: i32 = 0;
    
    if args.len() < 2 {
        eprintln!("ERROR: too few arguments");
        std::process::exit(1);
    }

    // variables for word content
    let mut words: Vec<String> = Vec::new();
    let mut word_occurrences: HashMap<String, i32> = HashMap::new();
    let mut bigram_occurrences: HashMap<String, i32> = HashMap::new();
    let mut trigram_occurrences: HashMap<String, i32> = HashMap::new();
    let mut quadgram_occurrences: HashMap<String, i32> = HashMap::new();
    let mut pentagram_occurrences: HashMap<String, i32> = HashMap::new();

    for i in 1..args.len() {
        let file_path = &args[i];

        // check if path exists
        if !(Path::new(file_path).exists()) {
            eprintln!("ERROR: cannot access \"{}\"", file_path);
            continue;
        }

        let file_extension = get_extension_from_filename(&file_path).unwrap();

        if !(file_extension == "txt") {
            eprintln!("ERROR: {} has unsupported filetype", file_path);
            continue;
        }

        let file_words = read_words_from_file(file_path)?; // get raw words
        let file_filtered_words = file_words.iter().filter(|w| w.len() >= 2).collect::<Vec<&String>>();

        for word in file_filtered_words.clone() {
            words.push(word.to_string());
            let cnt = word_occurrences.entry(word.to_string()).or_insert(0);
            *cnt+= 1;
        }

        let file_bigram_occurrences = get_ngram_occurrences(&file_words, 2)?;
        let file_trigram_occurrences = get_ngram_occurrences(&file_words, 3)?;
        let file_quadgram_occurrences = get_ngram_occurrences(&file_words, 4)?;
        let file_pentagram_occurrences = get_ngram_occurrences(&file_words, 5)?;

        for (key, value) in file_bigram_occurrences.clone() {
            let count = bigram_occurrences.entry(key).or_insert(0);
            *count+=value;
        }

        for (key, value) in file_trigram_occurrences.clone() {
            let count = trigram_occurrences.entry(key).or_insert(0);
            *count+=value;
        }
        for (key, value) in file_quadgram_occurrences.clone() {
            let count = quadgram_occurrences.entry(key).or_insert(0);
            *count+=value;
        }
        for (key, value) in file_pentagram_occurrences.clone() {
            let count = pentagram_occurrences.entry(key).or_insert(0);
            *count+=value;
        }

        valid_documents += 1;
    }

    // count number of bigram / trigrams
    let mut bigram_count: i32 = 0;
    let mut trigram_count: i32 = 0;
    let mut quadgram_count: i32 = 0;
    let mut pentagram_count: i32 = 0;
    
    for (_, count) in bigram_occurrences.clone() {
        bigram_count += count;
    }

    for (_, count) in trigram_occurrences.clone() {
        trigram_count += count;
    }
    for (_, count) in quadgram_occurrences.clone() {
        quadgram_count += count;
    }
    for (_, count) in pentagram_occurrences.clone() {
        pentagram_count += count;
    }
    println!("Number of valid documents: {}", valid_documents);
    println!("Number of words: {}", words.len());
    println!("Number of unique words: {}", word_occurrences.len());
    println!("Number of \"interesting\" bigrams: {}", bigram_count);
    println!("Number of unique \"interesting\" bigrams: {}", bigram_occurrences.len());
    println!("Number of \"interesting\" trigrams: {}", trigram_count);
    println!("Number of unique \"interesting\" trigrams: {}", trigram_occurrences.len());
    println!("Number of \"interesting\" 4-grams: {}", quadgram_count);
    println!("Number of unique \"interesting\" 4-grams: {}", quadgram_occurrences.len());
    println!("Number of \"interesting\" 5-grams: {}", pentagram_count);
    println!("Number of unique \"interesting\" 5-grams: {}\n", pentagram_occurrences.len());

    // sort
    let mut words_sorted: Vec<_> = word_occurrences.into_iter().collect();
    words_sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let mut bigram_sorted: Vec<_> = bigram_occurrences.into_iter().collect();
    bigram_sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let mut trigram_sorted: Vec<_> = trigram_occurrences.into_iter().collect();
    trigram_sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let mut quadgram_sorted: Vec<_> = quadgram_occurrences.into_iter().collect();
    quadgram_sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let mut pentagram_sorted: Vec<_> = pentagram_occurrences.into_iter().collect();
    pentagram_sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    match words_sorted.len() {
        1 => println!("Top 1 word:"),
        2..=127 => println!("Top {} words:", words_sorted.len()),
        _ => println!("Top 128 words:"),
    }

    for (key, value) in words_sorted.iter().take(128) {
        println!("{} {}", value, key);
    } 
    println!(""); 

    match bigram_sorted.len() {
        1 => println!("Top 1 interesting bigram:"),
        2..=63 => println!("Top {} interesting bigrams:", bigram_sorted.len()),
        _ => println!("Top 64 interesting bigrams:"),
    }

    for (bigram, count) in bigram_sorted.iter().take(64) {
        println!("{} {}", count, bigram);
    }
    println!("");   

    match trigram_sorted.len() {
        1 => println!("Top 1 interesting trigram:"),
        2..=31 => println!("Top {} interesting trigrams:", trigram_sorted.len()),
        _ => println!("Top 32 interesting trigrams:"),
    }

    for(trigram, count) in trigram_sorted.iter().take(32) {
        println!("{} {}", count, trigram);
    } 
    println!("");
    
    match quadgram_sorted.len() {
        1 => println!("Top 1 interesting 4-gram:"),
        2..=15 => println!("Top {} interesting 4-grams:", quadgram_sorted.len()),
        _ => println!("Top 16 interesting 4-grams:"),
    }

    for(quadgram, count) in quadgram_sorted.iter().take(16) {
        println!("{} {}", count, quadgram);
    }
    println!("");

    match pentagram_sorted.len() {
        1 => println!("Top 1 interesting 5-gram:"),
        2..=7 => println!("Top {} interesting 5-grams:", pentagram_sorted.len()),
        _ => println!("Top 8 interesting 5-grams:"),
    }

    for(pentagram, count) in pentagram_sorted.iter().take(8) {
        println!("{} {}", count, pentagram);
    }

    Ok(()) 
}
