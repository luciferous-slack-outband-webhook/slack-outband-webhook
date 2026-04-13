use serde::Serialize;
use worker::*;

#[derive(Serialize)]
struct HelloResponse {
    msg: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .post_async("/", handle_post)
        .run(req, env)
        .await
}

async fn handle_post(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::from_json(&HelloResponse {
        msg: "Hello, World!".to_string(),
    })
}
