use std::fs;
use std::path::PathBuf;

fn main() {
    let files = fs::read_dir(".").expect("不正なパス");
    for ent in files {
        let entry = ent.unwrap();
        let path: PathBuf = entry.path();
        let fname = path.to_str().unwrap_or("不正なファイル名");
        println!("{}", fname);
    }
}
