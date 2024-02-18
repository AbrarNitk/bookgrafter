#[tokio::main]
async fn main() {
    let settings = service::Settings::new(None);
    service::server::http_main(settings)
        .await
        .expect("error to start service");
}
