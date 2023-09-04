use std::{thread, time};

fn sleep_print(name: &str) {
    for i in 1..=3 {
        println!("{}: i={}", name, i);
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn main() {
    // スレッドなしの場合
    println!("--- スレッドなし ---");
    sleep_print("スレッドなし");

    // スレッドを使う場合
    println!("--- スレッドを利用 ---");
    thread::spawn(|| {
        sleep_print("次郎");
    });
    thread::spawn(|| {
        sleep_print("三郎");
    });
    sleep_print("太郎");
}
