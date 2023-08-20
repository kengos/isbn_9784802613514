fn main() {
    let price = 3950;
    let coin_500 = 10;
    let coin_100 = 3;
    let coin_50 = 10;

    for i_500 in 0..(coin_500 + 1) {
        for i_100 in 0..(coin_100 + 1) {
            for i_50 in 0..(coin_50 + 1) {
                let total = 500 * i_500 + 100 * i_100 + 50 * i_50;
                if total == price {
                    println!("500円x{}枚, 100円x{}枚, 50円x{}枚", i_500, i_100, i_50);
                }
            }
        }
    }
}
