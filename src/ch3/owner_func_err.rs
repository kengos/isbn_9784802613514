// error[E0382]: borrow of moved value: `g1`
//  --> owner_func_err.rs:4:20
//   |
// 2 |     let g1 = String::from("過ちを見過ごす人は美しい");
//   |         -- move occurs because `g1` has type `String`, which does not implement the `Copy` trait
// 3 |     show_message(g1); // 所有権が移動される
//   |                  -- value moved here
// 4 |     println!("{}", g1); // エラーになる
//   |                    ^^ value borrowed here after move
//   |
// note: consider changing this parameter type in function `show_message` to borrow instead if owning the value isn't necessary
//  --> owner_func_err.rs:7:26
//   |
// 7 | fn show_message(message: String) {
//   |    ------------          ^^^^^^ this parameter takes ownership of the value
//   |    |
//   |    in this function
//   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider cloning the value if the performance cost is acceptable
//   |
// 3 |     show_message(g1.clone()); // 所有権が移動される
//   |                    ++++++++
fn main() {
    let g1 = String::from("過ちを見過ごす人は美しい");
    show_message(g1); // 所有権が移動される
    println!("{}", g1); // エラーになる
}

fn show_message(message: String) {
    println!("{}", message);
}
