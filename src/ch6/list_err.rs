pub struct Node {
    data: i64,
    link: Option<Node>,
}

fn main() {
    let mut c = Node {
        data: 30,
        link: None,
    };
    println!("{}", c.data);
}

// error[E0072]: recursive type `Node` has infinite size
//  --> list_err.rs:1:1
//   |
// 1 | pub struct Node {
//   | ^^^^^^^^^^^^^^^
// 2 |     data: i64,
// 3 |     link: Option<Node>,
//   |                  ---- recursive without indirection
//   |
// help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
//   |
// 3 |     link: Option<Box<Node>>,
//   |                  ++++    +

// error: aborting due to previous error

// For more information about this error, try `rustc --explain E0072`.
