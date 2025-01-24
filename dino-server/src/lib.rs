mod config;
mod engine;
mod error;
mod middleware;
mod router;

use std::collections::HashMap;

use anyhow::Result;
use axum::{
    body::Bytes,
    extract::{Host, Query, State},
    http::{request::Parts, Response},
    response::IntoResponse,
    routing::any,
    Router,
};
use dashmap::DashMap;
use error::AppError;
use indexmap::IndexMap;
use matchit::Match;
use middleware::ServerTimeLayer;
use tokio::net::TcpListener;

pub use config::*;
pub use engine::*;
pub use router::*;
use tracing::info;

type ProjectRoutes = IndexMap<String, Vec<ProjectRoute>>;

#[derive(Clone)]
pub struct AppState {
    routers: DashMap<String, SwappableAppRouter>,
}

#[derive(Clone)]
pub struct TenentRouter {
    host: String,
    router: SwappableAppRouter,
}
pub async fn start_server(port: u16, routers: Vec<TenentRouter>) -> Result<()> {
    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on {}", listener.local_addr()?);
    let map = DashMap::new();
    for TenentRouter { host, router } in routers {
        map.insert(host, router);
    }
    let state = AppState::new(map);
    let app = Router::new()
        .route("/*path", any(handler))
        .layer(ServerTimeLayer)
        .with_state(state);
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[allow(unused)]
// only support json request and json response
async fn handler(
    State(state): State<AppState>,
    parts: Parts,
    Host(mut host): Host,
    Query(query): Query<HashMap<String, String>>,
    body: Option<Bytes>,
) -> Result<impl IntoResponse, AppError> {
    let router = get_router_by_host(host, state)?;
    let matched = router.match_it(parts.method.clone(), parts.uri.path())?;
    let req = assemble_req(&matched, &parts, query, body)?;
    let handler = matched.value;
    let worker = JsWorker::try_new(&router.code)?;
    let res = worker.run(handler, req)?;

    Ok(Response::from(res))
}

impl AppState {
    fn new(routers: DashMap<String, SwappableAppRouter>) -> Self {
        Self { routers }
    }
}

impl TenentRouter {
    pub fn new(host: impl Into<String>, router: SwappableAppRouter) -> Self {
        Self {
            host: host.into(),
            router,
        }
    }
}

fn get_router_by_host(mut host: String, state: AppState) -> Result<AppRouter, AppError> {
    let _ = host.split_off(host.find(':').unwrap_or(host.len()));
    info!("host:{}", host);

    let router = state
        .routers
        .get(&host)
        .ok_or(AppError::HostNotFound(host.to_string()))?
        .load_handler();

    Ok(router)
}

fn assemble_req(
    matched: &Match<&str>,
    parts: &Parts,
    query: HashMap<String, String>,
    body: Option<Bytes>,
) -> Result<Req, AppError> {
    let params: HashMap<String, String> = matched
        .params
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    let headers = parts
        .headers
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
        .collect();
    let body = body.and_then(|v| String::from_utf8(v.into()).ok());

    let req = Req::builder()
        .method(parts.method.to_string())
        .url(parts.uri.to_string())
        .query(query)
        .params(params)
        .headers(headers)
        .body(body)
        .build();
    Ok(req)
}
