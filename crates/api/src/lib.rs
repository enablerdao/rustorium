//! API層
//! 
//! axum、tonic、async-graphqlを使用した高性能なAPIサーバーを提供します。

use anyhow::Result;
use axum::{
    Router,
    routing::{get, post},
    extract::Json,
    response::IntoResponse,
};
use tonic::{transport::Server, Request, Response, Status};
use async_graphql::{Schema, EmptySubscription, Object};
use tracing::{info, warn, error};

/// APIサーバー
pub struct ApiServer {
    rest_router: Router,
    grpc_server: Server,
    graphql_schema: Schema<Query, Mutation, EmptySubscription>,
}

impl ApiServer {
    /// 新しいAPIサーバーを作成
    pub async fn new() -> Result<Self> {
        info!("Initializing API server...");
        
        // RESTルーターの設定
        let rest_router = Router::new()
            .route("/", get(health_check))
            .route("/api/v1/transactions", post(submit_transaction))
            .route("/api/v1/blocks", get(get_blocks));
        
        // gRPCサーバーの設定
        let grpc_server = Server::builder()
            .add_service(proto::node_server::NodeServer::new(NodeService::default()));
        
        // GraphQLスキーマの設定
        let graphql_schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .finish();
        
        Ok(Self {
            rest_router,
            grpc_server,
            graphql_schema,
        })
    }
    
    /// サーバーを起動
    pub async fn start(&mut self) -> Result<()> {
        info!("Starting API server...");
        
        // RESTサーバーの起動
        tokio::spawn(async move {
            axum::Server::bind(&"0.0.0.0:9071".parse().unwrap())
                .serve(self.rest_router.into_make_service())
                .await
                .unwrap();
        });
        
        // gRPCサーバーの起動
        tokio::spawn(async move {
            self.grpc_server
                .serve("0.0.0.0:9072".parse().unwrap())
                .await
                .unwrap();
        });
        
        // GraphQLサーバーの起動
        tokio::spawn(async move {
            axum::Server::bind(&"0.0.0.0:9073".parse().unwrap())
                .serve(
                    Router::new()
                        .route("/graphql", post(graphql_handler))
                        .into_make_service()
                )
                .await
                .unwrap();
        });
        
        info!("API server started successfully");
        Ok(())
    }
    
    /// サーバーを停止
    pub async fn stop(&mut self) -> Result<()> {
        info!("Stopping API server...");
        
        // TODO: 各サーバーの正常な停止処理
        
        info!("API server stopped successfully");
        Ok(())
    }
}

/// ヘルスチェックハンドラー
async fn health_check() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}

/// トランザクション送信ハンドラー
async fn submit_transaction(Json(tx): Json<Transaction>) -> impl IntoResponse {
    // TODO: トランザクション処理の実装
    Json(json!({ "status": "accepted", "hash": "0x..." }))
}

/// ブロック取得ハンドラー
async fn get_blocks() -> impl IntoResponse {
    // TODO: ブロック取得の実装
    Json(json!({ "blocks": [] }))
}

/// gRPCサービス
#[derive(Default)]
struct NodeService;

#[tonic::async_trait]
impl proto::node_server::Node for NodeService {
    async fn get_status(
        &self,
        request: Request<proto::StatusRequest>,
    ) -> Result<Response<proto::StatusResponse>, Status> {
        // TODO: ステータス取得の実装
        Ok(Response::new(proto::StatusResponse {
            status: "ok".to_string(),
        }))
    }
}

/// GraphQLクエリ
#[derive(Default)]
struct Query;

#[Object]
impl Query {
    async fn blocks(&self) -> Vec<Block> {
        // TODO: ブロック取得の実装
        vec![]
    }
    
    async fn transactions(&self) -> Vec<Transaction> {
        // TODO: トランザクション取得の実装
        vec![]
    }
}

/// GraphQLミューテーション
#[derive(Default)]
struct Mutation;

#[Object]
impl Mutation {
    async fn submit_transaction(&self, tx: Transaction) -> Result<TxHash> {
        // TODO: トランザクション送信の実装
        Ok(TxHash([0; 32]))
    }
}

/// GraphQLハンドラー
async fn graphql_handler(
    schema: Schema<Query, Mutation, EmptySubscription>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_api_lifecycle() -> Result<()> {
        let mut api = ApiServer::new().await?;
        
        // 起動テスト
        api.start().await?;
        
        // 停止テスト
        api.stop().await?;
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_rest_endpoints() -> Result<()> {
        let app = Router::new()
            .route("/", get(health_check))
            .route("/api/v1/transactions", post(submit_transaction))
            .route("/api/v1/blocks", get(get_blocks));
        
        let client = reqwest::Client::new();
        
        // ヘルスチェックテスト
        let resp = client.get("http://localhost:9071/").send().await?;
        assert_eq!(resp.status(), 200);
        
        Ok(())
    }
}
