pub async fn write(path: &camino::Utf8Path, content: &[u8]) -> Result<(), std::io::Error> {
    use tokio::io::AsyncWriteExt;
    let mut file = create_if_not(path).await?;
    file.write_all(content).await
}

pub async fn create_if_not(path: &camino::Utf8Path) -> std::io::Result<tokio::fs::File> {
    if path.exists() {
        tokio::fs::File::options()
            .append(true)
            .write(true)
            .open(path)
            .await
    } else {
        if let Some(parent) = path.parent() {
            create_dir_all(parent).await?;
        }
        tokio::fs::File::options()
            .create(true)
            .append(true)
            .write(true)
            .open(path)
            .await
    }
}

pub async fn create_dir_all(path: &camino::Utf8Path) -> std::io::Result<()> {
    if !path.exists() && path.is_dir() {
        tokio::fs::create_dir_all(path).await?;
    }
    Ok(())
}
