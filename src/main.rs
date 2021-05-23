use std::io::{self, Read};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rock", about = "A simple command line standard out parser")]
struct Opt {
    /// -s, --split,    rock --split ,
    #[structopt(short, long)]
    split: bool,

    /// -r, --replace,  rock --replace "~" "/home/jake"
    #[structopt(short, long)]
    replace: bool,

    first: Option<String>,
    second: Option<String>,
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
            current_part.push('\n');
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
    let mut output: String = String::new();

    if args.split {
        if args.first != None {
            let first_char: char = args.first.unwrap().chars().nth(0).unwrap();
            output = split(&input, first_char)
                .iter()
                .map(|x| x.to_owned())
                .collect();
        }
    } else if args.replace {
        if args.first != None && args.second != None {
            output = input.replace(&args.first.unwrap(), &args.second.unwrap())
        }
    }
    println!("{}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_test() {
        assert_eq!(
            split("this/will/get/split", '/'),
            vec!["this\n", "will\n", "get\n", "split"]
        );

        assert_eq!(
            split("com.jake.java.something", '.'),
            vec!["com\n", "jake\n", "java\n", "something"]
        );
    }
}
