use mini_grep::{run, Cli};
use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    match run(args){
        Ok(_) => { println!("Finished successfully") }
        Err(err) => {
            println!("Error: {}", err);
            std::process::exit(1);
        }
    }
}