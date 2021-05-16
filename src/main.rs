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
}

fn replace(literal: &str, from: &str, to: &str) -> String {
    return literal.replace(from, to).to_string();
}

fn split(literal: &str, separator: char) -> Vec<String> {
    let chars: Vec<char> = literal.chars().collect();

    let mut parts: Vec<String> = Vec::new();
    let mut index: usize = 0;

    let mut current_part = String::new();
    while index <= parts.len() {
        let current_char = chars[index];
        if current_char == separator {
            current_part.push(current_char);
            index += 1;
        } else {
            current_part = String::new();
            parts.push(current_part.to_owned());
        }
    }
    return parts;
}

fn main() -> io::Result<()> {
    let args = Opt::from_args();

    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    // Remove newline
    buffer.pop();
    print!("{}", buffer);
    Ok(())
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
            replace("this will not", "/", " "),
            "this will not"
        );
    }

    #[test]
    fn split_test() {
        assert_eq!(
            split("this/will/get/split", '/'),
            vec!["this", "will", "get", "replaced"]
        );
    }
}
