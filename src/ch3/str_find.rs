// 文字列の検索
fn main() {
    let s = "隣の客はよく柿食う客だ";

    // 柿を検索
    match s.find('柿') {
        Some(i) => println!("柿={}B", i),
        None => println!("柿はなし"),
    }
    // "バナナ"を検索
    match s.find("バナナ") {
        Some(i) => println!("バナナ={}B", i),
        None => println!("バナナはなし"),
    }
}
