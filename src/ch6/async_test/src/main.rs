#[tokio::main]
async fn main() {
    let f = say_later("諦めるのに時がある。");

    println!("捜すのに時がある。");
    f.await;
}

async fn say_later(msg: &'static str) {
    println!("{}", msg);
}
