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

// 構造体更新記法
fn main() {
    let taro = Person::new("Taro", 18);
    // JiroはTaroを複製して名前だけ変えたい
    let jiro = Person {
        name: String::from("Jiro"),
        ..taro // 構造体更新記法
    };
    // TaroとJiroを表示
    println!("{}, {}", taro.name, taro.age);
    println!("{}, {}", jiro.name, jiro.age);
}
