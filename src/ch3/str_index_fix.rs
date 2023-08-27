fn main() {
    let s: &str = "こんにちは";
    // 先頭の1文字を取り出して表示
    let ch = s.chars().nth(0).unwrap();
    println!("{}", ch); // こ

    // 0から数えて2番目を取り出して表示
    let ch = s.chars().nth(2).unwrap();
    println!("{}", ch); // に
}
