fn main() {
    // シャドーイングを使わない例
    {
        let mut v = 300; // vをmutableにする
        v = v + 5;
        println!("{}", v);
    }
    // シャドーイングを使う例
    {
        let v = 300; // vをimutableにする
        let v = v + 5;
        println!("{}", v);
    }
}
