fn main() {
    let s = "気前よく与えて豊かになる人がいる".to_string();
    echo(s); // 所有権が移動される
    println!("{}", s);
}

fn echo(s: String) {
    println!("{}", s);
}
