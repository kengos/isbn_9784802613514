fn main() {
    let pr = "猫に小判";
    // 1バイトずつ表示
    for c in pr.bytes() {
        print!("{:2x} ", c);
    }
    // バイト数を得る(&str.len() は バイト数を返す点に注意)
    println!("\nバイト数={}B", pr.len());
}
