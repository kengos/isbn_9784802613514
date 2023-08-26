// println!マクロは所有権の問題がでない
fn main() {
    let s = "気前よく与えて豊かになる人がいる".to_string();
    println!("{}", s);
    println!("{}", s);
}
