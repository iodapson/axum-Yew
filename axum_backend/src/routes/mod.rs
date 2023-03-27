mod return_json_data;

use axum::Extension;
use axum::{routing::get, Router};
use return_json_data::return_json_data;

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

// * get handler function of concern
// * axum::Extension is used to wrap the target data to be shared - SharedData
pub async fn access_shared_data(Extension(extracted_shared_data): Extension<SharedData>) -> String {
    extracted_shared_data.data_one
}
