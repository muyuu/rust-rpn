use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::{App, Arg};

fn main() {
    let matches = App::new("My RPN program")
        .version("1.0.0")
        .author("muyuu")
        .about("Super awesome sample RPN calculator")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .required(false),
        )
        .arg(
            Arg::new("formula_file")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .get_matches();

    if let Some(path) = matches.value_of("formula_file") {
        let f = File::open(path).expect("File 404");
        let reader = BufReader::new(f);

        for line in reader.lines() {
            let line = line.unwrap();
            println!("{}", line);
        }
    } else {
        println!("No file is specified");
    }

    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified?: {}", verbose);
}
