pub async fn http_main(bind: &str, port: u16) -> std::io::Result<()> {
    tracing_with_fmt();

    let listener = tokio::net::TcpListener::bind(std::net::SocketAddr::new(
        bind.parse().expect("unexpected value for `bind` address"),
        port,
    ))
    .await?;

    tracing::info!("### Server started at: {} ###", port);
    axum::serve(listener, crate::router::routes().await).await?;
    Ok(())
}

fn tracing_with_fmt() {
    use tracing_subscriber::prelude::*;
    let fmt_layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}
