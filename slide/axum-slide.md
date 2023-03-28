### axum (0.6.11)

The Rust backend framework that focuses on ergonomics and modularity. axum is designed to work with tokio and hyper.

Documentation: https://docs.rs/axum/latest/axum/

#### axum evolution

Check out https://github.com/tokio-rs/axum/blob/main/axum/CHANGELOG.md to learn more.

You can also take a look previous iterations of axum here - https://crates.io/crates/axum/versions

#### High-level features

- Route requests to handlers with a macro-free API.

- Declaratively parse requests using extractors.

- Simple and predictable error handling model.

- Generate responses with minimal boilerplate.

- Take full advantage of the tower ( https://crates.io/crates/tower ), and tower-http ( https://crates.io/crates/tower-http ) ecosystem of middleware, services, and utilities.

#### Required dependencies

To use axum, there are a few required dependencies that you have to pull in. They are:

```
[dependencies]
axum = "<latest-version>"
tokio = { version = "<latest-version>", features = ["full"] }
```

'hyper::Server' is re-exported by axum so if that's all you need you don't have to explicitly depend on hyper.

#### A quick taste of axum

The axum API you would see here simple returns a String response of "Hello World!" to the console.

Create a new Rust binary project using Cargo. Call the project 'first_axum_api'. Enter the following command;

```
$ cargo new first_axum_api
```

Next, make sure you have the following crates installed for your 'first_axum_api' project by running the following commands;

Add axum:

```
$ cargo add axum
```

Add hyper:

```
$ cargo add hyper -F full
```

Add tokio:

```
$ cargo add tokio -F full
```

or

```
$ cargo add tokio -F macros -F rt-multi-thread
```

Add tower:

```
$ cargo add tower
```

Add `tracing`

```
$ cargo add tracing
```

Add `tracing-subscriber`

```
$ cargo add tracing-subscriber
```

Now type the following code snippet into 'main.rs'

```rust
use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialise tracing
    tracing_subscriber::fmt::init();

    // build our application with a single route for the root-path of our application
    let app = Router::new().route("/", get( || async { "Hello, World" }));

    // Let's create a socket to serve our api from
    let addr = SocketAddr::from(([0,0,0,0], 3001));

    tracing::debug!("listening on {}", addr);

    // run it with hyper on localhost:3001
    axum::Server::bind(&addr) // alternate syntax: &"0.0.0.0:3001".parse().unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

Now run the app using the `$ cargo run` command and hit 'http://localhost:3001' on your browser or by using any preferred HTTP client.

Alternatively, you could `$ cargo watch -x run` to make your axum project automatically build and run itself everytime you change something inside your source and save it.

#### Notable Backend Concepts implemented in Rust tokio/axum

- Defining routes

- extractors

- returning a response

- sharing data between routes

- middlewares

- error handling

- CORS

- Database operations

##### defining routes

A route is an explicit mapping of a pair of url or API end-point to a specified backend service. A backend-service is a functionality which you define.

Routing is done in axum via the `axum::Router` struct (https://docs.rs/axum/latest/axum/struct.Router.html). Your service would require routing methods such as `axum::routing::get`, `axum::routing::post`, e.t.c. and a handler closure/function.

Take a look at the example in the section 'A quick taste of axum' and see if you can identify the route in it, its HTTP routing method (get), and its handler closure.

##### extractors

Extractors are how you pick apart the incoming request to get the parts your handler needs. Commonly headers can be found inside `axum::extract`, `axum::headers`, and `axum::http::header`.
They always run in the order of the function parameters that is, from left to right.
Here is an example extractor inside a request handler function called 'path':

```rust
use axum::extract::{Path, Query Json};
use std::collections::HashMap;

async fn path( Path(user_id): Path<u32>) {}

async fn json( Json(payload): Json<serde_json::Value>) {}
```

You can define your own custom extractor. Any type that implements 'FromRequest' or 'FromRequestParts' is an axum extractor.
The difference between 'FromRequestParts' and 'FromRequest' is that you'd implement 'FromRequestParts' if you extractor doesn't need access to the request body, whereas, if your extractor needs to consume the request body (e.g, by delaying with a timeout), then you must implement 'FromRequest'.

Here are examples of both traits being implemented:

###### implementing `FromRequest`

```rust
use axum::{
    async_trait,
    extract::FromRequest,
    response::{Response, IntoResponse}
    body::Bytes,
    routing::get,
    Router,
    http::{
        StatusCode,
        header::{HeaderValue, USER_AGENT},
        Request,
    },
};

struct CustomExtractor(String);

#[async_trait]
impl<S, B> FromRequest<S, B> for CustomExtractor
where
    Bytes: FromRequest<S, B>,
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = Response;

    // implement the required 'FromRequest' function 'from_request'
    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;

        // your custom extrator's logic goes here ... you can mutate body to fit your business logic

        Ok(Self(body))
    }
}

// Time to use 'CustomExtrator' inside a request handler
async fn handler(CustomExtractor(body): CustomExtractor) {
  // ...
}

let app = Router::new().route("/a_get_handler", get(handler));
```

Please note that since a request body is an asynchronous stream that can only be consumed once, you can only have one consumer extractor (like what you just saw) that consumes the request body. Therefore, for this reason, axum enforces that a consumer extractor must be the very last argument your handler takes.

###### implementing `FromRequestParts`

```rust
use axum::{
    async_trait,
    extract::FromRequestParts,
    routing::get,
    Router,
    http::{
      StatusCode,
      header::{HeaderValue, USER_AGENT},
      requests::Parts,
    },
}

struct ExtractUserAgent(HeaderValue);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractUserAgent
where
    S: Send + Sync
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(paths: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        if let Some(user_agent) = parts.headers.get(USER_AGENT) {
            Ok(ExtractUserAgent(user_agent.clone()))
        } else {
          Err((StatusCode::BAD_REQUEST, "`User-Agent` header is missing"))
        }
    }
}

async fn handler(ExtractUserAgent(user_agent): ExtractUserAgent) {
  // ...
}

let app = Router::new().route("/foo", get(handler));

```

N.B: You should not implement `FromRequestParts` and `FromRequest` together for the same type. Doing this would invalidate your custom extractor, unless the custom type is a wrapper for another extractor. For a comprehensive detail about extractors in axum, review https://docs.rs/axum/latest/axum/extract/index.html#structs

Below is an example of writing an extractor that generically wraps another extractor (by implementing both `FromRequest` and `FromRequestsParts`):

```rust
use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts},
    http::{request::Parts, Request},
    Json,
};
use serde::{Deserialize, Serialize};
//use std::time::{Duration, Instant};

// an extractor that wraps another and measures how long time it takes to run
#[derive(Deserialize, Serialize, Debug)]
pub struct TargetData<E> {
    extractor: E,
    pub field_one: String,   // * could be 'username' for example
    pub field_two: i32,      // * could be some form of tracker or even an 'id'
    pub field_three: String, // * 'String_three' could be renamed to 'password' for example
}
// we must implement both `FromRequestParts`
#[async_trait]
impl<S, T> FromRequestParts<S> for TargetData<T>
where
    S: Send + Sync,
    T: FromRequestParts<S>,
{
    type Rejection = T::Rejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        //let start = Instant::now();
        let extractor = T::from_request_parts(parts, state).await?;
        //let duration = start.elapsed();
        Ok(TargetData {
            extractor,
            field_one: String::from("field_one"),
            field_two: 2 as i32,
            field_three: String::from("field_three"),
        })
    }
}

// and `FromRequest`
#[async_trait]
impl<S, B, T> FromRequest<S, B> for TargetData<T>
where
    B: Send + 'static,
    S: Send + Sync,
    T: FromRequest<S, B>,
{
    type Rejection = T::Rejection;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        //let start = Instant::now();
        let extractor = T::from_request(req, state).await?;
        //let duration = start.elapsed();
        Ok(TargetData {
            extractor,
            field_one: String::from("field_one"),
            field_two: 2 as i32,
            field_three: String::from("string_three"),
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SampleJson {
    one: String,
    two: i32,
}

pub async fn custom_json_extractor(
    // this uses the `FromRequestParts` impl
    target_data: TargetData<Json<SampleJson>>, // this uses the `FromRequest` impl
) -> Json<SampleJson> {
    dbg!(&target_data.extractor.one);

    Json(SampleJson {
        one: String::from("one"),
        two: 2 as i32,
    })
}

```

##### returning a response

Any type that implements 'IntoResponse' can be returned as a response by axum. There would be little need to implement IntoResponse manually because axum provides implementations for many common types. It could be necessary though to implement 'IntoResponse' a custom error type when you want to return the custom error type as a response from handlers

```rust
use axum::{
    body::Body,
    routing::get,
    response::Json,
    Router,
};
use serde_json::{Value, json};

// `&'static str` becomes a `200 OK` with `content-type: text/plain; charset=utf-8`
async fn plain_text() -> &'static str {
    "foo"
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

let app = Router::new()
    .route("/plain_text", get(plain_text))
    .route("/json", get(json));
```

#### sharing data between routes

Sharing data between routes in axum is easy. You simply create the data you want to share, typically a struct of fields, derive 'Clone' for it, and then take `axum::Extension` as a parameter wrapping the target data inside the handler function you wish to access the shared data.

Since code is best explained with pseudo-code or code, here is a code example:

```rust
use axum::Extension;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing;
use tracing_subscriber;

#[derive(Clone)]
pub struct SharedData {
    pub data_one: String,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // instantiate (initialize) SharedData
    let shared_data = SharedData {
        data_one: "I am shared data one (1)".to_owned(),
    };

    // build our application with a single route for the root-path of our application
    let app = Router::new()
        .route("/", get(|| async { "Hello, World" }))
        .route("/access_shared_data", get(access_shared_data)) // * route of concern
        .layer(Extension(shared_data));

    // Let's create a socket to serve our api from
    let addr = SocketAddr::from(([0, 0, 0, 0], 3003));

    tracing::debug!("listening on {}", addr);

    // run it with hyper on localhost:3001
    axum::Server::bind(&addr) // alternate syntax: &"0.0.0.0:3001".parse().unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// * get handler function of concern
// * axum::Extension is used to wrap the target data to be shared - SharedData
pub async fn access_shared_data(Extension(extracted_shared_data): Extension<SharedData>) -> String {
    extracted_shared_data.data_one
}
```

###### Please Note:

You can share data between routes using State too. To see how, visit the front-page of axum documentation at docs.rs here: https://docs.rs/axum/0.6.12/axum/

#### middlewares

A middleware is a pre-built piece of code that adds features to your app akin to a browser-plugin. A middleware should work seamlessly with your app to be useful.

axum does not have its own bespoke middleware system and instead integrates with tower. This means the ecosystem of tower and tower-http middleware all work with axum.

Check this resource to learn more about middlewares, refer to: https://docs.rs/axum/0.6.12/axum/middleware/index.html

#### error handling

axum is based on `tower::Service` which bundles errors through its associated `Error` type. If you have a `Service` (basically in this case, a request handler) that produces an error and that error makes it all the way up to hyper, the connection will be terminated without a response. This is generally not desirable so axum makes sure you always produce a response by relying on the type system.

axum does this by requiring all services have `Infallible` ( https://doc.rust-lang.org/nightly/core/convert/enum.Infallible.html ) as their error type. `Infallible` is the error type for errors that can never happen.

This means if you define a handler like:

```rust
use axum::http::StatusCode;

async fn handler() -> Result<String, StatusCode> {
    // ...
}
```

While it looks like it might fail with a `StatusCode`, this actually isn't an error. If this handler returns 'Err(some_status_code), that will still be converted into a Response, and sent back to the client. This is done through `StatusCode`' `IntoResponse` implementation. These are not considered errors in axum.
This applies to extractors too. If an extractor doesn't match the request, the request will be rejected and a response will be returned without calling your handler. See this resource to learn about handling extractor failures - https://docs.rs/axum/latest/axum/extract/index.html

You could implement `IntoResponse` on a struct type, and use it to denote an error type.
!To-Do: Confirm this claim.

#### CORS (Cross-Origin Resource Sharing)

This is how you provide protection for your back-end API from being accessed by random requests that do not originate from within the backend-API itself. You specify which backend API endpoints you wish to expose and thus grant external access to using CORS.
!To-Do: Peer-review this definition/explanation

<b>N.B</b>: You need to add crate `tower-http` alongside its `cors` feature to enable cors inside your API.

```
$ cargo add tower-http -F cors
```

Here's a simple CORS snippet;

```rust
mod get_path_post_handler
mod send_path_post_handler;

use tower_http::cors::{Any, CorsLayer};
use axum::Router;
use axum::http::Method;
use axum::{routing::get, routing::post};

pub fn all_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route(
            "/receive",
            get(get_path_get_handler)
        )
        .route(
            "/send",
            post(send_path_post_handler)
        )
       // adds the CorsLayer as a middleware
       .layer(cors)
}
```

##### Database operations

<b>SQLx</b>

SQLx is an async, pure Rust + SQL crate featuring compile-time checked queries without a DSL.

To learn more about SQLx, use the following resources:

Lib.rs: https://lib.rs/crates/sqlx

Crates.io: https://docs.rs/sqlx/latest/sqlx/index.html

<b>SeaORM</b>

SeaORM is a relational ORM to help you build web services in Rust with the familiarity of dynamic languages.

To learn more about SeaOR, utilize the following resources:

docs.rs: https://docs.rs/sea-orm/latest/sea_orm/

SeaORM Homepage: https://www.sea-ql.org/SeaORM/

<b>Diesel</b>

Diesel is a safe, extensible ORM and query builder for Rust. It calls itself the most productive way to interact with databases in Rust because of its safe and composable abstractions over queries.

Find resources to learn about Diesel here:

Official Page: https://diesel.rs

#### Sample axum API that returns json data when aN HTTP GET request is made to it

Please refer to the binary project called `axum_backend` accompanying this repo.

#### axum ecosystem

Discord:
Matrix: https://matrix.to/#/#tokio-rs/axum:matrix.org

<b>Thank you!!</b>
