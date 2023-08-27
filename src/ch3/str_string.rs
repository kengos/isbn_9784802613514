// &str と String の相互変換
fn main() {
    let ss: &str = "気前よく与えるなら豊かになる";
    // &str から String(= Vec<u8>) への変換
    let so1: String = String::from(ss);
    let so2: String = ss.to_string();
    // String から &str への変換
    let ss2: &str = &so1;
    // so2.as_str() でも同じポインターアドレスになる
    let ss3: &str = so1.as_str();
    // 画面に表示
    println!("{}\n{}\n{}\n{}", so1, so2, ss2, ss3);
    // 参照型ポインターアドレスを表示
    println!("{:p}\n{:p}", ss2, ss3);
}
