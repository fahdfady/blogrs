use blog_api::{
    config::{SERVER_HOST, SERVER_PORT},
    db, routes,
};
use sqlx::{migrate, Sqlite};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let pool: sqlx::Pool<Sqlite> = db::connect().await;

    migrate!()
        .run(&pool)
        .await
        .expect("unable to run database migrations");

    let app: axum::Router = routes::create_router(pool);

    let addr: SocketAddr = SocketAddr::new(SERVER_HOST.into(), SERVER_PORT);

    println!("Server running at http://{}", addr);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .expect("server failed to start :(");
}
