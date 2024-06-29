use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use anyhow::Result;
use tokio::net::TcpListener;

mod database;
mod layer;
mod model;
mod router;
mod service;

#[tokio::main]
async fn main() -> Result<()> {
    let db = database::database_connection();
    let app = router::all_router(Arc::new(db));
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    let listener = TcpListener::bind(&address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
