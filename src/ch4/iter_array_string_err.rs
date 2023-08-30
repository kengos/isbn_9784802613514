fn main() {
    let array = [
        "Apple".to_string(),
        "Banana".to_string(),
        "Mango".to_string(),
        "Tomato".to_string(),
    ];

    for a in array {
        // ここで所有権が移動する
        println!("{}", a);
    }

    println!("len={}", array.len());
}

// error[E0382]: borrow of moved value: `array`
//    --> iter_array_string_err.rs:14:24
//     |
// 2   |     let array = [
//     |         ----- move occurs because `array` has type `[String; 4]`, which does not implement the `Copy` trait
// ...
// 9   |     for a in array {
//     |              ----- `array` moved due to this implicit call to `.into_iter()`
// ...
// 14  |     println!("len={}", array.len());
//     |                        ^^^^^^^^^^^ value borrowed here after move
//     |
// note: `into_iter` takes ownership of the receiver `self`, which moves `array`
//    --> /usr/local/rustup/toolchains/1.71.1-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/traits/collect.rs:271:18
//     |
// 271 |     fn into_iter(self) -> Self::IntoIter;
//     |                  ^^^^
// help: consider iterating over a slice of the `[String; 4]`'s content to avoid moving into the `for` loop
//     |
// 9   |     for a in &array {
