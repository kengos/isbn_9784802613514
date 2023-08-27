// ライフタイム
// &'static str のみを指定できる関数
fn echo(s: &'static str) {
    println!("{}", s);
}

fn main() {
    echo("愚かな人でも黙ってみていると");
    echo("賢いと見られる");

    // 以下のコメント部分は失敗する
    //     error[E0597]: `s` does not live long enough
    //     --> str_static.rs:13:10
    //      |
    //   12 |     let s = String::from("テスト");
    //      |         - binding `s` declared here
    //   13 |     echo(&s);
    //      |     -----^^-
    //      |     |    |
    //      |     |    borrowed value does not live long enough
    //      |     argument requires that `s` is borrowed for `'static`
    //   14 | }
    //      | - `s` dropped here while still borrowed

    //   error: aborting due to previous error
    // let s = String::from("テスト");
    // echo(&s);
}
