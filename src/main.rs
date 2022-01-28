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
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", answer);
    }
}

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    // 文字列を一つずつ取り出せるようベクタ形式にする
    // その際，pop() で取り出したいので逆順にする
    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    // ベクタを取り出してスタックへ入れつつ計算する
    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        0
    }
}
