use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("A", 30);
    map.insert("B", 50);
    // キーが存在するか確認する
    if map.get("D") == None {
        println!("Dは存在しない");
    } else {
        println!("D={}", map["D"]);
    }
}
