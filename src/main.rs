#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let router = atbb::run().await?;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
