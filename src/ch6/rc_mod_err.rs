// Rc<T> は変更不可
use std::rc::Rc;

fn main() {
    let mut a_rc = Rc::new(1000);
    let mut b_rc = Rc::clone(&a_rc);
    *b_rc += 100; // 変更不可
    println!("{}", b_rc);
}

// warning: variable does not need to be mutable
//  --> rc_mod_err.rs:4:9
//   |
// 4 |     let mut a_rc = Rc::new(1000);
//   |         ----^^^^
//   |         |
//   |         help: remove this `mut`
//   |
//   = note: `#[warn(unused_mut)]` on by default

// warning: variable does not need to be mutable
//  --> rc_mod_err.rs:5:9
//   |
// 5 |     let mut b_rc = Rc::clone(&a_rc);
//   |         ----^^^^
//   |         |
//   |         help: remove this `mut`

// error[E0594]: cannot assign to data in an `Rc`
//  --> rc_mod_err.rs:6:5
//   |
// 6 |     *b_rc += 100; // 変更不可
//   |     ^^^^^^^^^^^^ cannot assign
//   |
//   = help: trait `DerefMut` is required to modify through a dereference, but it is not implemented for `Rc<i32>`

// error: aborting due to previous error; 2 warnings emitted

// For more information about this error, try `rustc --explain E0594`.
