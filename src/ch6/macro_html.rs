macro_rules! out_html {
    // 引数なしの場合
    () => {()};
    // 引数が1つだけの場合
    ($e:tt) => {
        println!("{}", $e);
    };
    // タグ　[ 内側 ] 続きを指定した場合
    (
        $tag:ident [ $($inner:tt)* ] $($rest:tt)*
    ) => {{
        print!("<{}>", stringify!($tag));
        out_html!($($inner)*);
        println!("</{}>", stringify!($tag));
        out_html!($($rest)*);
    }}
}

fn main() {
    out_html!(
        html [
            head[title["test"]]
            body[
                h1["test"]
                p ["This is test."]
            ]
        ]
    );
}
