pub async fn http_main(settings: service::Settings) -> std::io::Result<()> {
    tracing_with_fmt();

    let listener = tokio::net::TcpListener::bind(std::net::SocketAddr::new(
        settings
            .bind
            .parse()
            .expect("unexpected value for `bind` address"),
        settings.port,
    ))
    .await?;

    let context = service::context::new(&settings);
    tracing::info!("### Server started at: {} ###", settings.port);
    axum::serve(listener, crate::router::routes(context).await).await?;
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
