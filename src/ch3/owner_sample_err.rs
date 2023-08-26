// error[E0382]: borrow of moved value: `g1`
//  --> owner_sample_err.rs:4:20
//   |
// 2 |     let g1 = String::from("穏やかな心は体に良い");
//   |         -- move occurs because `g1` has type `String`, which does not implement the `Copy` trait
// 3 |     let g2 = g1; // 所有権をg2へ移動
//   |              -- value moved here
// 4 |     println!("{}", g1);
//   |                    ^^ value borrowed here after move
//   |
//   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider cloning the value if the performance cost is acceptable
//   |
// 3 |     let g2 = g1.clone(); // 所有権をg2へ移動
//   |                ++++++++
//
// error: aborting due to previous error; 1 warning emitted
fn main() {
    let g1 = String::from("穏やかな心は体に良い");
    let g2 = g1; // 所有権をg2へ移動
    println!("{}", g1);
}
