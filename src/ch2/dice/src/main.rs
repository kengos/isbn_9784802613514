use rand::Rng;

fn main() {
    // 乱数の生成器を用意
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let dice = rng.gen_range(1..=6);
        println!("{}", dice);
    }
}
