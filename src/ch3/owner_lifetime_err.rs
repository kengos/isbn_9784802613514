// error[E0106]: missing lifetime specifier
//  --> owner_lifetime_err.rs:1:21
//   |
// 1 | fn gen_message() -> &str {
//   |                     ^ expected named lifetime parameter
//   |
//   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
// help: consider using the `'static` lifetime
//   |
// 1 | fn gen_message() -> &'static str {
//   |                      +++++++
//
// error: aborting due to previous error
fn gen_message() -> &str {
    let msg = String::from("過ちを見過ごす人は美しい");
    return &msg;
}

fn main() {
    let m = gen_message();
    println!("{}", m);
}
