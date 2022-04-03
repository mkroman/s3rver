use std::net::SocketAddr;

use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing::{debug, error};

pub async fn start_server() {
    debug!("Starting HTTP server");

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/*path", get(handlers::get))
        .layer(TraceLayer::new_for_http());

    // run our app with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("http serve failed");

    error!("server quit unexpectedly");
}

mod handlers {
    use axum::extract::Path;

    pub(crate) async fn get(Path(path): Path<String>) -> String {
        path
    }
}
