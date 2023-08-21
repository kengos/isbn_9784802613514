fn main() {
    let mut v: usize = 10;
    set_value(&mut v);
    println!("v = {}", v);
}

fn set_value(arg: &mut usize) {
    *arg = 100;
}
