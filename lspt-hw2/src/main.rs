use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::ffi::OsStr;
use std::collections::HashMap;

fn parse_html_tag(input: &str) -> String {
    let mut output = String::new();
    let mut in_tag = false;
    for char in input.chars() {
        if char == '<'
        {
            in_tag = true;
        }
        else if char == '>'
        {
            in_tag = false;
        }
        else if !in_tag
        {
            output.push(char);
        }
    }
    if output.len() < 2 {
        return String::new();
    }
    sanitize(output)
}


fn sanitize(check: String) -> String {
    let temp: String = check.chars()
        .filter(|&c| c != '\n' && c != '\t' && c != '\r')
        .collect();
    temp.replace(|c| !char::is_alphanumeric(c), " ")
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

fn read_words_from_file(file_path: &str) -> io::Result<Vec<(String, i32)>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let mut words = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.len() < 2 {
            continue;
        }
        for word in line.split_whitespace().collect::<Vec<&str>>() {
            words.push(word.to_string());
        }
    }
    let mut word_count = HashMap::new();
    for word in words {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    let mut sorted_by_value: Vec<_> = word_count.into_iter().collect();
    sorted_by_value.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(sorted_by_value.into_iter().map(|(word, count)| (word.clone(), count)).collect())
}

fn read_words_from_html_file(file_path: &str) -> io::Result<Vec<(String, i32)>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut words = Vec::new();

    
    for line in reader.lines() {
        let line = line?;
        if line.len() < 2 {
            continue;
        }
        for word in parse_html_tag(&line).split_whitespace().collect::<Vec<&str>>() {
            words.push(word.to_string());
        }
    }
    let mut word_count = HashMap::new();
    for word in &words {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    let mut sorted_by_value: Vec<_> = word_count.into_iter().collect();
    sorted_by_value.sort_by(|a, b| b.1.cmp(&a.1));
    Ok(sorted_by_value.into_iter().map(|(word, count)| (word.clone(), count)).collect())
}



fn main() -> io::Result<()> {
    let file_path = "/home/ben-dennison/Documents/GitHub/LSPT-HW2/lspt-hw2/src/text.html";
    let file_extension = get_extension_from_filename(&file_path).unwrap();
    println!("Reading words from filetype: {}", get_extension_from_filename(file_path).unwrap());
    if file_extension == "html" {
        println!("HTML file detected");
        let words = read_words_from_html_file(file_path)?;
        for (word, count) in words {
            println!("{}: {}", count, word);
        }
    }
    else if file_extension == "txt" {
        println!("TXT file detected");
        let words = read_words_from_file(file_path)?;
        for (word, count) in words {
            println!("{}: {}", count, word);
        }
    }
    else {
        println!("Filetype not supported");
    }
    Ok(())
}
