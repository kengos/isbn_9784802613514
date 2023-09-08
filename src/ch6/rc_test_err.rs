fn main() {
    let a_box = Box::new(100);
    {
        // 所有権を移動
        let b_box = a_box;
        println!("{}", b_box);
    }
    // ここでは利用できない
    println!("{}", a_box);
}

// error[E0382]: borrow of moved value: `a_box`
//  --> rc_test_err.rs:9:20
//   |
// 2 |     let a_box = Box::new(100);
//   |         ----- move occurs because `a_box` has type `Box<i32>`, which does not implement the `Copy` trait
// ...
// 5 |         let b_box = a_box;
//   |                     ----- value moved here
// ...
// 9 |     println!("{}", a_box);
//   |                    ^^^^^ value borrowed here after move
//   |
//   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
// help: consider cloning the value if the performance cost is acceptable
//   |
// 5 |         let b_box = a_box.clone();
//   |                          ++++++++

// error: aborting due to previous error

// For more information about this error, try `rustc --explain E0382`.
