mod data_sharing;
mod return_json;

use axum::Extension;
use axum::{routing::get, Router};
use data_sharing::access_shared_data;
use return_json::return_json_data;

#[derive(Clone)]
pub struct SharedData {
    pub data_one: String,
}

pub fn build_routes() -> Router {
    // instantiate (initialize) SharedData
    let shared_data = SharedData {
        data_one: "I am shared data one (1)".to_owned(),
    };

    // build our application with a single route for the root-path of our application
    Router::new()
        .route("/access_shared_data", get(access_shared_data)) // * route of concern
        .layer(Extension(shared_data))
        .route("/return-json-data", get(return_json_data))
        .route(
            "/",
            get(|| async { "You're using axum. Try this endpoint: '/return-json-data'" }),
        )
}
