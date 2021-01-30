use std::{error::Error, fs};
use structopt::StructOpt;
use std::io::Write;

#[derive(StructOpt, Debug)]
pub struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    #[structopt(short, long)]
    ignore_case: bool,
    #[structopt(short, long)]
    #[allow(clippy::option_option)]
    output: Option<String>,
}

// returns either OK or Err with a type with the Error trait
pub fn run(args: Cli) -> Result<(), Box<dyn Error>> {
    // if fs::read_to_string returns an Err it is directly returned as the result of the run function thanks to the ?
    let contents = fs::read_to_string(&args.path)?;

    let results = if args.ignore_case {
        search_case_insensitive(&args.pattern, &contents)
    } else {
        search(&args.pattern, &contents)
    };

    // write the result to the file if it is specified with the -o or --output flag
    if let Some(out) = args.output {
        let mut output = fs::File::create(&out)?;
        results.iter().cloned().try_for_each(|a| {
            match output.write((a.to_owned() + "\n").as_bytes()) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        })?;
        println!("Search result was saved in the file: {}", out);
        return Ok(());
    }

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

// you have to indicate the shelf life to remove any cramping, the return and a slice of the content so they must have the same shelf life
pub fn search<'a>(pattern: &String, contents: &'a String) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

pub fn search_case_insensitive<'a>(pattern: &String, contents: &'a String) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&pattern.to_lowercase()))
        .collect()
}