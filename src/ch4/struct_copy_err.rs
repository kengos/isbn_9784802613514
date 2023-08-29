struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: &str, age: i32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

// error[E0382]: borrow of moved value: `taro`
//   --> struct_copy_err.rs:21:35
//    |
// 16 |     let taro = Person::new("taro", 18);
//    |         ---- move occurs because `taro` has type `Person`, which does not implement the `Copy` trait
// 17 |     // JiroはTaroを複製して名前だけ変えたい
// 18 |     let mut jiro = taro;
//    |                    ---- value moved here
// ...
// 21 |     println!("{}, {}", taro.name, taro.age);
//    |                                   ^^^^^^^^ value borrowed here after move
//    |
//    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

// error: aborting due to previous error
fn main() {
    let taro = Person::new("taro", 18);
    // JiroはTaroを複製して名前だけ変えたい
    let mut jiro = taro;
    jiro.name = String::from("jiro");
    // TaroとJiroを表示
    println!("{}, {}", taro.name, taro.age);
    println!("{}, {}", jiro.name, jiro.age);
}
