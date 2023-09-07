pub struct Node {
    data: i64,
    link: Option<Box<Node>>,
}

// Box::new(value) => ヒープ領域に確保
fn node(v: i64, link: Option<Box<Node>>) -> Option<Box<Node>> {
    Some(Box::new(Node { data: v, link }))
}

fn main() {
    // cはスタック領域, cの指す先はヒープ領域
    let c = node(10, node(20, node(30, None))).unwrap();
    let mut p = &c;
    loop {
        println!("{}", p.data);
        match p.link {
            None => break,
            Some(ref link) => p = &link,
        }
    }
}
