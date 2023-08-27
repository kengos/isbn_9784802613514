use encoding_rs;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    // 保存ファイル名を指定
    let filename = "test-sjis.txt";
    // Shift-JISで保存
    save_sjis(filename, "こっそり食べる物はおいしい。");
    // Shift-JIS を読み込み
    let s = load_sjis(filename);
    println!("{}", s);
}

fn save_sjis(filename: &str, text: &str) {
    // Shift-JISへエンコード
    let (enc, _, _) = encoding_rs::SHIFT_JIS.encode(text);
    let buf = enc.into_owned();
    // ファイルにバイナリーを保存
    let mut file = File::create(filename).expect("作成");
    file.write(&buf[..]).expect("書込");
}

fn load_sjis(filename: &str) -> String {
    // ファイルからバイト列を一気に読み込む
    let buf = fs::read(filename).expect("読込");
    // Shift-JISでデコード
    let (dec, _, _) = encoding_rs::SHIFT_JIS.decode(&buf);
    return dec.into_owned();
}
