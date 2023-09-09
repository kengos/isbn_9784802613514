peg::parser! ( grammar calc() for str {
    // ルートとなる規則を追加
    pub rule eval() -> i64
        = expr()

    // 足し算と引き算を行うルールを追加
    rule expr() -> i64
        = l:term() "+" r:expr() { l + r }
        / l:term() "-" r:expr() { l + r }
        / term()

    rule term() -> i64
        = l:value() "*" r:term() { l * r }
        / l:value() "/" r:term() { l / r }
        / v:value()

    rule value() -> i64
        = number()
        / "(" v:expr() ")" { v }

    // 数値を読む規則を追加
    rule number() -> i64
        = n:$(['0'..='9']+) // 構文定義
        { n.parse().unwrap() }
});

fn main() {
    println!("{}", calc::eval("1+2*3").unwrap());
    println!("{}", calc::eval("(1+2)*3").unwrap());
    println!("{}", calc::eval("100/2-1").unwrap());
}
