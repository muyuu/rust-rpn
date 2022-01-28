use std::fs::File;
use std::io::{BufRead, BufReader, stdin};
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

    let verbose = matches.is_present("verbose");

    if let Some(path) = matches.value_of("formula_file") {
        let f = File::open(path).expect("File 404");
        let reader = BufReader::new(f);

        run(reader, verbose);
    } else {
        // ファイル指定がない場合は標準入力を受け付ける
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, verbose);
    }
}

fn run<R: BufRead>(reader:R, verbose: bool) {
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{}", line);
    }
}

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        0
    }
}
