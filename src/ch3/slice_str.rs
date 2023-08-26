fn main() {
    let s = String::from("beep");
    let ss = &s[0..3];
    // スライスの内容を表示
    println!("{}", ss);
}
