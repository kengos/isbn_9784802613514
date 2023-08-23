fn main() {
    let s = "3.1415";
    let num = s.parse::<f64>().expect("変換に失敗");
    println!("{:.2}", num);
}
