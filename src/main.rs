use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rock", about = "A simple command line standard out parser")]
struct Opt {
    /// -s, --split,    rock --split ,
    #[structopt(long = "split", short = "s")]
    split: bool,

    /// -r, --replace,  rock --replace ~ /home/jake
    #[structopt(long = "replace", short = "r")]
    replace: bool
}

fn main() {
    let args = Opt::from_args();

    println!("{:?}", args);
}
