fn main() {
    let mut g1 = String::from("過ちを見過ごす人は美しい");
    g1 = show_message(g1); // 所有権が移動される
    println!("{}", g1); // エラーになる
}

// Stringを受け取りStringを返す関数
fn show_message(message: String) -> String {
    println!("{}", message);
    return message;
}
