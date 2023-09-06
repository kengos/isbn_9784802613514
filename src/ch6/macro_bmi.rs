macro_rules! bmi_select {
    (
        $bmi:expr;
        $( $label:expr => $range:expr );+
    ) => {{
        let mut result = "error";
        $ (
            if $range.start <= $bmi && $bmi < $range.end {
                result = $label;
            }
        ) +
        result
    }};
}

fn main() {
    let h: f32 = 158.0;
    let w: f32 = 63.0;
    let bmi = w / (h / 100.0).powf(2.0);
    let label = bmi_select![
        bmi;
        "低体重" => 0.0..18.5;
        "普通" => 18.5..25.0;
        "肥満1度" => 25.0..30.0;
        "肥満2度" => 30.0..35.0;
        "肥満3度" => 35.0..40.0;
        "肥満4度" => 40.0..99.9
    ];
    println!("判定: {}", label);
}
