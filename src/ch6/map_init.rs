macro_rules! map_init {
    ( $($key:expr => $val:expr), *) => {{
        let mut tmp = std::collections::HashMap::new();
        $ (
            tmp.insert($key, $val);
        ) *
        tmp
    }}
}

fn main() {
    let week = map_init![
        "mon" => "月曜日",
        "tue" => "火曜日",
        "wed" => "水曜日",
        "thu" => "木曜日",
        "fri" => "金曜日",
        "sat" => "土曜日",
        "sun" => "日曜日"
    ];
    println!("mon = {}", week["mon"]);
    println!("wed = {}", week["wed"]);
}
