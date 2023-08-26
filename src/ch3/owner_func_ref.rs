fn main() {
    let g1 = String::from("過ちを見過ごす人は美しい");
    show_message(&g1); // 参照を渡す
    println!("{}", g1); // 所有権が移動されていないので、エラーにならない
}

fn show_message(message: &String) {
    println!("{}", message);
}
