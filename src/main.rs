use std::io::{self, Read};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rock", about = "A simple command line standard out parser")]
struct Opt {
    /// -s, --split,    rock --split ,
    #[structopt(short, long)]
    split: bool,

    /// -r, --replace,  rock --replace ~ /home/jake
    #[structopt(short, long)]
    replace: bool,

    first: Option<String>,
    second: Option<String>,
}

fn replace(literal: &str, from: &str, to: &str) -> String {
    return literal.replace(from, to).to_string();
}

fn split(literal: &str, separator: char) -> Vec<String> {
    let chars: Vec<char> = literal.chars().collect();

    let mut parts: Vec<String> = Vec::new();
    let mut index: usize = 0;

    let mut current_part: String = String::new();
    while index < chars.len() {
        let current_char: char = chars[index];
        // If it runs into the thing being split by,
        // push to parts and create a new part
        if current_char == separator {
            parts.push(current_part);
            current_part = String::new();
        } else {
            current_part.push(current_char);
        }
        // If it has reached the end of the string,
        // add the last part to parts because there
        // are no more separators in the string
        if index + 1 == chars.len() {
            parts.push(current_part.clone());
        }
        index += 1;
    }
    return parts;
}

fn read_input() -> Result<String, String> {
    let mut buffer: String = String::new();
    let mut stdin = io::stdin();
    match stdin.read_to_string(&mut buffer) {
        Ok(_) => {
            // Remove newline
            buffer.pop();
            return Ok(buffer.to_string());
        }
        Err(e) => {
            eprintln!("{}", e);
            return Err(e.to_string());
        }
    }
}

fn main() {
    let args: Opt = Opt::from_args();

    let input: String = read_input().unwrap();
    println!("{:?}", split(&input, ' '));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_test() {
        assert_eq!(
            replace("this/will/get/replaced", "/", " "),
            "this will get replaced"
        );

        assert_eq!(
            replace("this will not replace", "/", " "),
            "this will not replace"
        );
    }

    #[test]
    fn split_test() {
        assert_eq!(
            split("this/will/get/split", '/'),
            vec!["this", "will", "get", "split"]
        );

        assert_eq!(
            split("com.jake.java.something", '.'),
            vec!["com", "jake", "java", "something"]
        );
    }
}
