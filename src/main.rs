use async_graphql_parser::types::ExecutableDocument;
use async_trait::async_trait;
use axum::{
    body::{self, BoxBody},
    extract::{FromRequest, RequestParts},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;

#[tokio::main]
async fn main() {
    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::EnvFilter::new(
    //         std::env::var("RUST_LOG").unwrap_or_else(|_| "graphql-test=debug".into()),
    //     ))
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    tracing::warn!("init");
    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .layer(ServiceBuilder::new().map_request_body(body::boxed));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn graphql_handler(body: ParsedGraphQLRequest) -> impl IntoResponse {
    "Hello, World!"
}

struct ParsedGraphQLRequest(ExecutableDocument);

#[async_trait]
impl FromRequest<BoxBody> for ParsedGraphQLRequest {
    type Rejection = Response;

    async fn from_request(request: &mut RequestParts<BoxBody>) -> Result<Self, Self::Rejection> {
        tracing::warn!("a");
        // let request = Request::from_request(request)
        //     .await
        //     .map_err(|err| err.into_response())?;
        let json_body = Json::<serde_json::Value>::from_request(request)
            .await
            .map_err(|err| err.into_response())?;
        tracing::warn!("body: {:?}", json_body);
        tracing::warn!("b");

        let body_string = match &json_body["query"] {
            serde_json::Value::String(body_string) => body_string,
            _ => panic!("expected query"),
        };
        // let (_parts, body) = request.into_parts();
        // let bytes = hyper::body::to_bytes(body)
        //     .await
        //     .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?;
        // tracing::warn!("c");
        // let body_string = String::from_utf8(bytes.to_vec()).unwrap();
        // tracing::warn!("body: {body_string}");
        Ok(ParsedGraphQLRequest(
            async_graphql_parser::parse_query(body_string)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?,
        ))
    }
}
