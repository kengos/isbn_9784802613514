fn main() {
    let height_cm = input("身長(cm)は? ");
    let weight = input("体重(kg)は? ");
    // BMIの計算
    let height = height_cm / 100.0;
    let bmi = weight / height.powf(2.0);

    println!("BMI={:.1}", bmi);

    // 判定結果を表示
    if bmi < 18.5 {
        println!("低体重");
    } else if bmi < 25.0 {
        println!("普通体重");
    } else if bmi < 30.0 {
        println!("肥満1度");
    } else if bmi < 35.0 {
        println!("肥満2度");
    } else if bmi < 40.0 {
        println!("肥満3度");
    } else {
        println!("肥満4度");
    }
}

// 標準入力から一行読んでf64型で返す関数
fn input(prompt: &str) -> f64 {
    // メッセージの表示
    println!("{}", prompt);
    // 入力を得る
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("入力エラー");
    // 空白を除去して数値に変換
    return s.trim().parse().expect("数値変換エラー");
}
