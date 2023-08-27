fn main() {
    let pr = "知恵は武器よりも価値がある。";
    let mut sub1 = String::new();
    for (i, c) in pr.chars().enumerate() {
        if i < 2 {
            sub1.push(c);
            continue;
        }
        break;
    }
    println!("先頭2文字: {}", sub1);

    // 「武器」の部分を得る
    let mut sub2 = String::new();
    for (i, c) in pr.chars().enumerate() {
        if 3 <= i && i <= 4 {
            sub2.push(c);
            continue;
        }
    }
    println!("4-5文字目: {}", sub2);
}
