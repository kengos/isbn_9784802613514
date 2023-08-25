use std::{env, path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut target_dir = ".";
    if args.len() >= 2 {
        // パスを指定した場合
        target_dir = &args[1];
    }
    // PathBufに変換
    let target = path::PathBuf::from(target_dir);
    println!("{}", target_dir);
    tree(&target, 0);
}

fn tree(target: &path::PathBuf, level: isize) {
    let files = target.read_dir().expect("存在しないパス");
    for ent in files {
        let path = ent.unwrap().path();
        // level分だけindent
        for _ in 1..=level {
            print!("|    ");
        }
        let fname = path.file_name().unwrap().to_string_lossy();
        if path.is_dir() {
            println!("|-- <{}>", fname);
            tree(&path, level + 1);
            continue;
        }
        // ファイル名を表示
        println!("|-- {}", fname);
    }
}
