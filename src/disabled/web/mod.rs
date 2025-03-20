use std::sync::Arc;
use std::net::SocketAddr;
use axum::{
    Router,
    routing::{get, post},
    extract::{State, Json},
    response::{IntoResponse, Response},
    http::{StatusCode, Uri, header},
    body::{boxed, Full},
};
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer,
};
use include_dir::{include_dir, Dir};
use mime_guess::from_path;
use serde::{Deserialize, Serialize};
use anyhow::Result;

use crate::core::{
    dag::{Transaction, TxId},
    storage::state::StateManager,
};

// フロントエンドファイルを埋め込む
static FRONTEND_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/frontend");

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub from: String,
    pub to: String,
    pub amount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    pub id: String,
    pub status: String,
    pub timestamp: i64,
}

/// ウェブサーバーの設定
#[derive(Debug, Clone)]
pub struct WebConfig {
    /// ホスト
    pub host: String,
    /// ポート
    pub port: u16,
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 53036,  // 提供されているポートを使用
        }
    }
}

/// アプリケーションの状態
#[derive(Clone)]
pub struct AppState {
    state_manager: Arc<StateManager>,
    api_port: u16,
    ws_port: u16,
    graphql_port: u16,
}

/// ウェブサーバー
pub struct WebServer {
    /// 設定
    config: WebConfig,
    /// アプリケーションの状態
    state: AppState,
}

impl WebServer {
    /// 新しいウェブサーバーを作成
    pub fn new(
        config: WebConfig,
        state_manager: Arc<StateManager>,
        api_port: u16,
        ws_port: u16,
        graphql_port: u16,
    ) -> Self {
        Self {
            config,
            state: AppState {
                state_manager,
                api_port,
                ws_port,
                graphql_port,
            },
        }
    }

    /// サーバーを起動
    pub async fn start(&self) -> Result<()> {
        // ルーターを作成
        let app = Router::new()
            // APIエンドポイント
            .route("/api/transactions", post(Self::create_transaction))
            .route("/api/transactions/:id", get(Self::get_transaction))
            // 静的ファイルを提供
            .route("/*path", get(Self::serve_static))
            // アプリケーションの状態を追加
            .with_state(self.state.clone())
            // CORSを設定
            .layer(
                CorsLayer::new()
                    .allow_origin(["http://localhost:3000".parse()?])
                    .allow_methods(tower_http::cors::Any)
                    .allow_headers(tower_http::cors::Any),
            )
            // トレーシングを追加
            .layer(TraceLayer::new_for_http());

        // アドレスを作成
        let addr = format!("{}:{}", self.config.host, self.config.port)
            .parse::<SocketAddr>()?;

        // サーバーを起動
        tracing::info!("Starting web server on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }

    /// トランザクションを作成
    async fn create_transaction(
        State(state): State<AppState>,
        Json(req): Json<CreateTransactionRequest>,
    ) -> impl IntoResponse {
        // トランザクションを作成
        let tx = Transaction {
            id: TxId::new(vec![0; 32]), // TODO: 適切なIDを生成
            from: req.from.into(),
            to: req.to.into(),
            amount: req.amount,
            timestamp: chrono::Utc::now().timestamp(),
        };

        // トランザクションを保存
        match state.state_manager.save_transaction(tx.clone()).await {
            Ok(_) => {
                let response = TransactionResponse {
                    id: hex::encode(tx.id.as_bytes()),
                    status: "pending".to_string(),
                    timestamp: tx.timestamp,
                };
                (StatusCode::CREATED, Json(response))
            }
            Err(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Error: {}", e)))
            }
        }
    }

    /// トランザクションを取得
    async fn get_transaction(
        State(state): State<AppState>,
        id: String,
    ) -> impl IntoResponse {
        // トランザクションIDをデコード
        let tx_id = TxId::from(hex::decode(id).unwrap());

        // トランザクションを取得
        match state.state_manager.get_transaction(&tx_id).await {
            Ok(Some(tx)) => {
                let response = TransactionResponse {
                    id: hex::encode(tx.id.as_bytes()),
                    status: "confirmed".to_string(), // TODO: 実際のステータスを取得
                    timestamp: tx.timestamp,
                };
                (StatusCode::OK, Json(response))
            }
            Ok(None) => {
                (StatusCode::NOT_FOUND, Json("Transaction not found".to_string()))
            }
            Err(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Error: {}", e)))
            }
        }
    }

    /// 静的ファイルを提供
    async fn serve_static(State(state): State<AppState>, uri: Uri) -> Response {
        let path = uri.path().trim_start_matches('/');
        let path = if path.is_empty() { "index.html" } else { path };

        if let Some(file) = FRONTEND_DIR.get_file(path) {
            let mime_type = from_path(path).first_or_octet_stream();
            let mut contents = String::from_utf8_lossy(file.contents()).into_owned();

            // HTMLファイルの場合、環境変数を置換
            if path.ends_with(".html") {
                contents = contents
                    .replace("{{API_BASE_URL}}", &format!("http://localhost:{}", state.api_port))
                    .replace("{{WS_BASE_URL}}", &format!("ws://localhost:{}", state.ws_port))
                    .replace("{{GRAPHQL_BASE_URL}}", &format!("http://localhost:{}", state.graphql_port));
            }

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime_type.as_ref())
                .body(boxed(Full::from(contents)))
                .unwrap()
        } else if path.contains('.') {
            // ファイルが見つからない場合は404を返す
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(boxed(Full::from("File not found")))
                .unwrap()
        } else {
            // SPAのルーティング用にindex.htmlを返す
            if let Some(file) = FRONTEND_DIR.get_file("index.html") {
                let mut contents = String::from_utf8_lossy(file.contents()).into_owned();
                contents = contents
                    .replace("{{API_BASE_URL}}", &format!("http://localhost:{}", state.api_port))
                    .replace("{{WS_BASE_URL}}", &format!("ws://localhost:{}", state.ws_port))
                    .replace("{{GRAPHQL_BASE_URL}}", &format!("http://localhost:{}", state.graphql_port));

                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "text/html")
                    .body(boxed(Full::from(contents)))
                    .unwrap()
            } else {
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(boxed(Full::from("index.html not found")))
                    .unwrap()
            }
        }
    }
}