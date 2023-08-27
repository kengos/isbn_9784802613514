// &str に対してスライスを使う
fn main() {
    let s = "abcdefg";
    println!("{}", &s[0..1]); // a
    let s = "こんにちは";
    let ch = &s[..3];
    println!("{}", ch); // こ

    let ch = &s[6..9];
    println!("{}", ch); // に
}
