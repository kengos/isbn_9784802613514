use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("findfile (path) (keyword)");
        return;
    }

    let target_file = &args[1];
    let keyword = &args[2];
    // PathBufに変換
    let target = path::PathBuf::from(target_file);
    findfile(&target, keyword);
}

fn findfile(target: &path::PathBuf, keyword: &str) {
    let files = target.read_dir().expect("存在しないパス");
    for dir_entry in files {
        // PathBufを得る
        let path = dir_entry.unwrap().path();
        // ディレクトリならば再帰的に検索
        if path.is_dir() {
            findfile(&path, keyword);
            continue;
        }
        // ファイル名を文字列に変換
        // to_string_lossy() => 表現できない文字列を U+FFFD に変換
        let fname = path.file_name().unwrap().to_string_lossy();
        if None == fname.find(keyword) {
            continue;
        }
        // キーワードを含むパスを表示
        println!("{}", path.to_string_lossy());
    }
}
