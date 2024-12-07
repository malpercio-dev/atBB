use atbb::Config;
use color_eyre::Help;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let port: u16 = std::env::var("BIND_PORT")
        .ok()
        .map(|p| p.parse())
        .unwrap_or(Ok(8080))
        .with_warning(|| "the provided BIND_PORT is not valid")
        .with_note(|| "the default port of 8080 is used if not specified")?;

    let addr: IpAddr = std::env::var("BIND_ADDR")
        .ok()
        .map(|p| p.parse())
        .unwrap_or(Ok(IpAddr::V4(Ipv4Addr::UNSPECIFIED)))
        .with_warning(|| "the provided BIND_ADDR is not valid")
        .with_note(|| "the default of `0.0.0.0` is used if not specified")?;

    let socket_addr: SocketAddr = (addr, port).into();

    let router = atbb::run(Config {
        database_kind: atbb::DatabaseKind::Sqlite,
    })
    .await?;
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
