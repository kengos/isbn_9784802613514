fn main() {
    let v1 = 0xFF; // 16進数で255を指定
    let v2 = 0o655; // 8進数で429を指定
    let v3 = 0b1101_0101; // 2進数で213を指定

    println!("{},{},{}", v1, v2, v3);
}
