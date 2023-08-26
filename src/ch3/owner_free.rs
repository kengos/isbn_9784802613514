fn main() {
    // ブロック
    {
        let s1 = String::from("聞かないで返事をする人は愚か");
        println!("{}", s1);
    }
    // ブロックを抜けるとs1は破棄される
}
