fn get_int_from_file() -> Result<i32, String> {
    let path = "number.txt";

    // ファイルがない場合はエラーオブジェクトを文字列にする
    // 最後の ? は Result 型を返す演算子
    // 直前の結果の Result 型の値が Ok(t) なら t を返し Err(e) なら Err(e) で早期リターン
    let num_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;

    num_str
        .trim()
        .parse::<i32>()
        // parse() の結果が Ok の場合は2倍して Ok(t * 2) となる
        .map(|t| t * 2)
        // parse() の結果が Err の場合は e の文字列を返して Err(e) となる
        .map_err(|e| e.to_string())
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}
