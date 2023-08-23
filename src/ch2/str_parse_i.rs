fn main() {
    let s = "365";
    let i: i32 = match s.parse() {
        Ok(v) => v,  // 成功
        Err(_) => 0, // 失敗
    };
    println!("{}", i);
}
