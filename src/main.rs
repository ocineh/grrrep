use structopt::StructOpt;
use grrrep::{run, Cli};

fn main() {
    let args = Cli::from_args();

    if let Err(err) = run(args){
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}