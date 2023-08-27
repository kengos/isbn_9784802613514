// 文字列の分割 - 特定インデックスで分割
fn main() {
    // 郵便番号を指定
    let zipcode = "105-0011";

    // スライスで分割
    println!("-- スライス --");
    println!("前半: {}", &zipcode[..3]);
    println!("後半: {}", &zipcode[4..]);

    // split_at で分割
    println!("-- split_at --"); // MEMO: 細かいがテキストだと "---" と "-" が一つ多い
    let (zip1, zip2) = zipcode.split_at(3);
    let (zip2, zip3) = zip2.split_at(1);
    println!("前半: {}", zip1);
    println!("記号: {}", zip2);
    println!("後半: {}", zip3);

    //  split_off で分割
    println!("-- split_off --");
    let mut zip1 = String::from(zipcode);
    let mut zip2 = zip1.split_off(3);
    let zip3 = zip2.split_off(1);
    println!("前半: {}", zip1);
    println!("記号: {}", zip2);
    println!("後半: {}", zip3);

    // splitで分割
    println!("-- split_off --");
    let zip_a: Vec<&str> = zipcode.split("-").collect();
    println!("前半: {}", zip_a[0]);
    println!("後半: {}", zip_a[1]);
}
