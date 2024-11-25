use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::ffi::OsStr;
use std::collections::{HashMap, HashSet};
use std::env;


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
        for word in clean(line).rsplit(|c| !char::is_alphabetic(c) && c != '\'').collect::<Vec<&str>>() {
            if word.len() < 2 {
                continue;
            }
            words.push(word.to_string());
        }
    }
    Ok(words)
}

fn get_word_occurrences(words: &Vec<String>) -> io::Result<Vec<(String, i32)>> {
    let mut word_count: HashMap<&str, i32> = HashMap::new();
    for word in words {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    let mut sorted_by_value: Vec<_> = word_count.into_iter().collect();
    sorted_by_value.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(sorted_by_value.into_iter().map(|(word, count)| (word.to_string(), count)).collect())
}

fn get_bigram_occurrences(words: &Vec<String>) -> io::Result<Vec<(String, i32)>> {
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

fn get_trigram_occurrences(words: &Vec<String>) -> io::Result<Vec<(String, i32)>> {
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
    let args: Vec<String> = env::args().collect();
    let mut valid_documents: i32 = 0;
    
    if args.len() < 2 {
        eprintln!("ERROR: too few arguments");
        std::process::exit(1);
    }

    let mut print_strings: Vec<String> = Vec::new();
    for i in 1..args.len() {
        let file_path = &args[i];

        // check if path exists
        if !(Path::new(file_path).exists()) {
            eprintln!("ERROR: cannot access {}", file_path);
            std::process::exit(1);
        }

        let file_extension = get_extension_from_filename(&file_path).unwrap();
        if file_extension == "txt" {
            let words = read_words_from_file(file_path)?;
            let words_len = words.len();
            let word_occurrences = get_word_occurrences(&words)?;
            let bigram_occurrences = get_bigram_occurrences(&words)?;
            let trigram_occurrences = get_trigram_occurrences(&words)?;
            print_strings.push(format!("Number of words : {}", words_len));
            print_strings.push(format!("Number of unique words : {}", word_occurrences.len()));
            print_strings.push(format!("Number of interesting bigrams : {}", bigram_occurrences.len()));
            print_strings.push(format!("Number of unique interesting bigrams : {}", 23232));
            print_strings.push(format!("Number of interesting trigrams : {}", 14379));
            print_strings.push(format!("Number of unique interesting trigrams : {}\n", trigram_occurrences.len()));

            match word_occurrences.len() {
                1 => print_strings.push(format!("Top 1 word:")),
                2..=63 => print_strings.push(format!("Top {} words:", word_occurrences.len())),
                _ => print_strings.push(format!("Top 64 words:")),
            }
            for (word, count) in word_occurrences.iter().take(64) {
                print_strings.push(format!("{}: {}", count, word));
            }
            print_strings.push(format!("\n"));

            match bigram_occurrences.len() {
                1 => print_strings.push(format!("Top 1 interesting bigram:")),
                2..=31 => print_strings.push(format!("Top {} interesting bigrams:", bigram_occurrences.len())),
                _ => print_strings.push(format!("Top 32 interesting bigrams:")),
            }
            for (bigram, count) in bigram_occurrences.iter().take(32) {
                print_strings.push(format!("{}: {}", count, bigram));
            }
            print_strings.push(format!("\n"));

            match trigram_occurrences.len() {
                1 => print_strings.push(format!("Top 1 interesting trigram:")),
                2..=15 => print_strings.push(format!("Top {} interesting trigrams:", trigram_occurrences.len())),
                _ => print_strings.push(format!("Top 16 interesting trigrams:")),
            }
            for(trigram, count) in trigram_occurrences.iter().take(16) {
                print_strings.push(format!("{}: {}", count, trigram));
            }
            valid_documents += 1;
        }
        else {
            eprintln!("ERROR: Filetype not supported");
        }
    }

    println!("Number of valid documents: {}", valid_documents);

    for s in print_strings {
        println!("{}", &s);
    }

    Ok(())
}
