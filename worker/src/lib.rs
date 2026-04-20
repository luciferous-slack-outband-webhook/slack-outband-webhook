mod slack;

use serde::Serialize;
use slack::{VerifyError, verify_slack_signature};
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

async fn handle_post(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let Some(timestamp) = req
        .headers()
        .get("X-Slack-Request-Timestamp")
        .ok()
        .flatten()
    else {
        return Response::error("Missing X-Slack-Request-Timestamp", 401);
    };

    let Some(signature) = req.headers().get("X-Slack-Signature").ok().flatten() else {
        return Response::error("Missing X-Slack-Signature", 401);
    };

    let body = match req.text().await {
        Ok(b) => b,
        Err(_) => return Response::error("Failed to read body", 400),
    };

    let signing_secret = match ctx.secret("SLACK_APP_SIGNING_SECRET") {
        Ok(s) => s.to_string(),
        Err(_) => return Response::error("Server configuration error", 500),
    };

    let now_seconds = Date::now().as_millis() / 1000;

    match verify_slack_signature(
        &timestamp,
        &signature,
        &body,
        signing_secret.as_bytes(),
        now_seconds,
    ) {
        Ok(()) => {}
        Err(VerifyError::TimestampParseError) => return Response::error("Invalid timestamp", 401),
        Err(VerifyError::TimestampTooOld) => return Response::error("Timestamp too old", 401),
        Err(VerifyError::InvalidSignatureFormat) | Err(VerifyError::InvalidHex) => {
            return Response::error("Invalid signature format", 401);
        }
        Err(VerifyError::SignatureMismatch) => return Response::error("Invalid signature", 401),
        Err(VerifyError::HmacInitError) => {
            return Response::error("Server configuration error", 500);
        }
    }

    Response::from_json(&HelloResponse {
        msg: "Hello, World!".to_string(),
    })
}
