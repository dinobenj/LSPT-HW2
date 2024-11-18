use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::Index;
use std::path::Path;
use std::ffi::OsStr;

fn sanitize(check: String) -> String {
    check.replace(|c| !char::is_alphanumeric(c), " ")
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
        for word in line.split_whitespace() {
            words.push(word.to_string());
        }
    }

    Ok(words)
}

fn read_words_from_html_file(file_path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut words = Vec::new();

    
    for line in reader.lines() {
        let line = line?;
        if line.len() < 2 {
            continue;
        }
        let sanitized = sanitize(line.to_string());
        let mut msg = sanitized.to_string();
        if let Some((m, _)) = msg.split_once("html") {
            msg = m.to_string();
        }
        words.push(sanitized);
    }
    Ok(words)
}

fn main() -> io::Result<()> {
    let file_path = "/Users/bendennison/Documents/lspt-hw2/src/test.html";
    let file_extension = get_extension_from_filename(&file_path).unwrap();
    println!("Reading words from filetype: {}", get_extension_from_filename(file_path).unwrap());
    if file_extension == "html" {
        println!("HTML file detected");
        let words = read_words_from_html_file(file_path)?;
        for word in words {
            println!("{}", word);
        }
    }
    else if file_extension == "txt" {
        println!("TXT file detected");
        let words = read_words_from_file(file_path)?;
        for word in words {
            println!("{}", word);
        }
    }
    else {
        println!("Filetype not supported");
    }
    Ok(())
}
