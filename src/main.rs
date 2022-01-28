use anyhow::{bail, ensure, Context, Result};
use clap::{App, Arg};
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

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

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("answer: {}", answer),
            Err(e) => eprintln!("{:#?}", e),
        }
    }

    Ok(())
}

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    // 文字列を一つずつ取り出せるようベクタ形式にする
    // その際，pop() で取り出したいので逆順にする
    pub fn eval(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    // ベクタを取り出してスタックへ入れつつ計算する
    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack = Vec::new();
        let mut pos = 0;

        // ベクタの最後から取り出し，
        // - 数値ならスタックに入れる
        // - 演算子ならスタックにある数値を2つ取りだして演算する
        // - 演算結果をスタックに入れる
        // ベクタが殻になって最後にスタックに残っている数値が結果になる
        while let Some(token) = tokens.pop() {
            pos += 1;

            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().context(format!("invalid syntax at {}", pos))?;
                let x = stack.pop().context(format!("invalid syntax at {}", pos))?;

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("invalid token at {}", pos),
                };
                stack.push(res);
            }

            // have -v option
            if self.0 {
                println!("{:?} {:?}", tokens, stack);
            }
        }

        ensure!(stack.len() == 1, "invalid syntax");
        Ok(stack[0])
    }
}

// #[...] の内側が cfg(...) の場合
// - cargo build ( run ) の際に直下の mod が無効化される
// - cargo test の場合は有効になる
#[cfg(test)]
mod tests {
    // use super::* は，この tests モジュールの親で定義されている
    // 構造体や関数をすべて使えるようになるキーワード
    use super::*;

    // #[test] アトリビュートでテストコードだと認識される
    #[test]
    fn test_ok() {
        let calc = RpnCalculator::new(false);
        assert_eq!(calc.eval("5"), 5);
        assert_eq!(calc.eval("20"), 20);
        assert_eq!(calc.eval("-10"), -10);

        assert_eq!(calc.eval("2 3 +"), 5);
        assert_eq!(calc.eval("2 3 *"), 6);
        assert_eq!(calc.eval("2 3 -"), -1);
        assert_eq!(calc.eval("2 3 /"), 0);
        assert_eq!(calc.eval("2 3 %"), 2);
    }

    // エラーでパニックを出していることをテストしたい場合は
    // #[should_panic] アトリビュートを使う
    #[test]
    #[should_panic]
    fn test_ng() {
        let calc = RpnCalculator(false);
        calc.eval("2 2 ^");
    }
}
