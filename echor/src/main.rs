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
            .action(clap::ArgAction::SetFalse),
        )
        .get_matches();

    let text: Vec<String> = _matches
        .get_many::<String>("text")
        .into_iter()
        .flatten()
        .cloned()
        .collect();
    let omit_newline = _matches.get_flag("omit_newline");

    // let ending = if omit_newline { "" } else { "\n" };

   
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });

    // println!("{:#?}", _matches);
}
