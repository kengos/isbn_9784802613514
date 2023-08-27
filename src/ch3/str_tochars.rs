// 文字列を1文字ずつ表示する
fn main() {
    let pr = "窮鼠猫を噛む";
    // 1文字ずつ表示
    for ch in pr.chars() {
        print!("[{}]", ch);
    }
    // 文字数を調べる
    println!("\n文字数={}字", pr.chars().count());

    // Vec<char> に変換して処理する
    let pr_chars: Vec<char> = pr.chars().collect();
    for ch in pr_chars.iter() {
        print!("({})", ch);
    }
    println!("\n文字数={}字", pr_chars.len());
}
