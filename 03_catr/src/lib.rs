use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config
{
    files: Vec<String>,
    number_lines: bool, 
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()>
{
    for filename in config.files
    {
        println!("{}", filename);
    }
    // dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config>
{
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Rob Samoraj <rsamoraj11@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input files")
                .multiple(true)
                .default_value("-")
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines")
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .conflicts_with("number_lines")
                .takes_value(false)   
        )
        .get_matches();

    Ok(Config 
        {
            files: matches.values_of_lossy("files").unwrap(),
            number_lines: matches.is_present("number_lines"),
            number_nonblank_lines: matches.is_present("number_nonblank_lines"),
        })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>>
{
    match filename
    {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}