fn main() {
    let a_vec: Vec<u32> = vec![100, 200, 300];
    for i in a_vec {
        println!("{}", i);
    }
    let s_vec: Vec<&str> = vec!["犬", "猫", "鶏"];
    for i in s_vec {
        println!("{}", i);
    }
}
