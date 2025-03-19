use axum::{
    http::StatusCode,
    routing::get_service,
    Router,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;
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
    let static_dir = PathBuf::from(".");
    
    // 静的ファイルを提供するサービス
    let static_service = get_service(ServeDir::new(&static_dir))
        .handle_error(|error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        });
    
    // CORSの設定
    let cors = CorsLayer::permissive();
    
    // ルーターの構築
    let app = Router::new()
        .nest_service("/", static_service.clone())
        .fallback_service(static_service)
        .layer(cors);
    
    // サーバーの起動
    // ポートを環境変数から取得するか、0を指定して空いているポートを使用
    let port = std::env::var("FRONTEND_PORT").unwrap_or_else(|_| "0".to_string()).parse().unwrap_or(0);
    
    // 通常のバインド方法
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let actual_addr = listener.local_addr()?;
    info!("Frontend server listening on {}", actual_addr);
    println!("Frontend server listening on {}", actual_addr);
    
    // 実際のポート番号をファイルに書き込む
    std::fs::write("/tmp/frontend_port", actual_addr.port().to_string())?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
