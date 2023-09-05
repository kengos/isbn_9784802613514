use actix_web::{web, App, HttpRequest, HttpServer};

// アドレスとポートの指定
const SERVER_ADDR: &str = "127.0.0.1:8888";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("[SERVER] http://{}/", SERVER_ADDR);
    HttpServer::new(|| {
        // ルーティングを指定
        App::new().route("/", web::get().to(index))
    })
    .bind(SERVER_ADDR)?
    .run()
    .await
}

// 実行される関数
async fn index(req: HttpRequest) -> &'static str {
    println!("request: {:?}", req);
    "Hello, World!"
}
