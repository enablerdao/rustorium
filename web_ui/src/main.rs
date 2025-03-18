use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get_service,
    Router,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::services::ServeDir;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ロギングの設定
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
    
    // 静的ファイルのディレクトリ
    let static_dir = PathBuf::from("./static");
    
    // 静的ファイルを提供するサービス
    let static_service = get_service(ServeDir::new(&static_dir))
        .handle_error(|error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        });
    
    // ルーターの構築
    let app = Router::new()
        .nest_service("/", static_service.clone())
        .fallback_service(static_service);
    
    // サーバーの起動
    let mut port = 57620; // 外部からアクセス可能なポートに変更
    let mut attempts = 0;
    let max_attempts = 10;
    
    // ポートが使用中の場合は別のポートを試す
    while attempts < max_attempts {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        match tokio::net::TcpListener::bind(&addr).await {
            Ok(listener) => {
                info!("Web UI server listening on {}", addr);
                println!("Web UI server listening on {}", addr);
                return axum::serve(listener, app).await.map_err(Into::into);
            },
            Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
                attempts += 1;
                port += 1;
                warn!("Port {} is already in use, trying port {}", port - 1, port);
            },
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    
    Err(anyhow::anyhow!("Failed to bind to a port after {} attempts", max_attempts))
    
    Ok(())
}
