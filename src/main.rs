use structopt::StructOpt;
use std::io::{self, Read};

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
