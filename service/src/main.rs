#[tokio::main]
async fn main() {
    service::server::http_main("0.0.0.0", 8000)
        .await
        .expect("error to start service");
}
