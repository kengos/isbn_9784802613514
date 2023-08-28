static mut TAX: f32 = 0.1;

fn main() {
    // use of mutable static is unsafe and requires unsafe function or block
    unsafe {
        // ミュータブルなstatic変数を使う
        println!("Price: {}", TAX * 300.0);
        // staticな変数を変更する
        TAX = 0.08;
        println!("Price: {}", TAX * 300.0);
    }
}
