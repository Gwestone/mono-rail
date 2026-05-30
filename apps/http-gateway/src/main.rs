mod http_router;

use axum::Router;
use config_lib::AppConfig;
use http_router::http_router;

#[tokio::main]
async fn main() -> Result<(), String> {
    //load configs
    let config = AppConfig::load()?;

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new().merge(http_router());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(config.server_url)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
