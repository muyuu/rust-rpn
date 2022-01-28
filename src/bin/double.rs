use anyhow::{Context, Result};

fn get_int_from_file() -> Result<i32> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read string from {}", path))?;

    num_str
        .trim()
        .parse::<i32>()
        // parse() の結果が Ok の場合は2倍して Ok(t * 2) となる
        .map(|t| t * 2)
        // parse() の結果が Err の場合は e の文字列を返して Err(e) となる
        .context("failed to parse string")
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{:#?}", e),
    }
}
