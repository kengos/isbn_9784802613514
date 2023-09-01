use std::io;

fn main() {
    let mut stack: Vec<f64> = vec![];
    println!("RPN: ");
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("入力エラー");
    let tokens = s.split_whitespace();
    for token in tokens {
        let t = token.trim();
        match t.parse::<f64>() {
            Ok(v) => {
                stack.push(v);
                continue;
            }
            Err(_) => 0.0,
        };
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();
        match t {
            "+" => stack.push(a + b),
            "-" => stack.push(a - b),
            "*" => stack.push(a * b),
            "/" => stack.push(a / b),
            _ => panic!("未定義の演算子: {}", t),
        };
    }
    println!("{}", stack.pop().unwrap());
}
