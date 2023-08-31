fn print_bmi(height: f32, weight: Option<f32>) {
    let bmi: Option<f32> = match weight {
        Some(w) => Some(w / (height / 100.0).powf(2.0)),
        None => None,
    };
    let msg = match bmi {
        Some(n) if n < 18.5 => "低体重",
        Some(n) if n < 25.0 => "標準体重",
        Some(n) if n < 30.0 => "肥満1度",
        Some(n) if n < 35.0 => "肥満2度",
        Some(n) if n < 40.0 => "肥満3度",
        Some(_) => "肥満4度",
        None => "測定不能",
    };
    println!("BMI={:.1}, 判定={}", bmi.unwrap_or(0.0), msg);
}

fn main() {
    let height = 163.3;
    print_bmi(height, Some(48.0));
    print_bmi(height, Some(72.3));
    print_bmi(height, None);
}
