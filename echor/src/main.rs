use clap::{Command, Arg};

fn main() 
{
    let _matches = Command::new("echor")
        .author("Rob Samoraj <rsamoraj11@gmail.com>")
        .version("1.0")
        .about("Rust echo command line utility")
        .arg(
            Arg::new("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)
            .num_args(1),
        )
        .arg(
            Arg::new("omit_newline")
            .short('n')
            .help("Do not print newline")
            .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    println!("{:#?}", _matches);
}
