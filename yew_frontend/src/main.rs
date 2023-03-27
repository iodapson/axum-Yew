use yew_frontend::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

// Serve this application with trunk command;
// $ trunk serve --proxy-backend=http://localhost:8080/return-json-data
