// 問題のあるプログラム
// もしもスライスで文字の途中を指定したら...
// thread 'main' panicked at 'byte index 1 is not a char boundary;
// it is inside '知' (bytes 0..3) of `知恵は武器よりも価値がある。`', str_slice_err.rs:3:21
fn main() {
    let pr = "知恵は武器よりも価値がある。";
    println!("{}", &pr[1..6]); // 文字の途中を取り出す
}
