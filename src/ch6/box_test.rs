fn main() {
    // ヒープ領域に100の値を確保して、ポインターを返す
    let x_box = Box::new(100);
    // デリファレンス(*) で値を取得
    let x_val = *x_box;
    println!("{}", x_val);
}
