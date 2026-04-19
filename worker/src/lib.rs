use hmac::{Hmac, Mac};
use serde::Serialize;
use sha2::Sha256;
use worker::*;

type HmacSha256 = Hmac<Sha256>;

/// タイムスタンプの許容範囲（秒）。Slack公式は5分を推奨。
const TIMESTAMP_TOLERANCE_SECONDS: u64 = 5 * 60;

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
    let _body = match verify_slack_signature(&mut req, &ctx).await {
        Ok(body) => body,
        Err(err_response) => return err_response,
    };

    Response::from_json(&HelloResponse {
        msg: "Hello, World!".to_string(),
    })
}

/// Slackリクエストの署名を検証する。
///
/// 成功時は `Ok(String)` でリクエストボディを返す。
/// 検証失敗時は `Err(Result<Response>)` で 401 レスポンスを返す。
async fn verify_slack_signature(
    req: &mut Request,
    ctx: &RouteContext<()>,
) -> std::result::Result<String, Result<Response>> {
    // 1. ヘッダーからタイムスタンプと署名を取得（ボディ読み取り前に行う）
    let timestamp = req
        .headers()
        .get("X-Slack-Request-Timestamp")
        .ok()
        .flatten()
        .ok_or_else(|| Response::error("Missing X-Slack-Request-Timestamp", 401))?;

    let signature = req
        .headers()
        .get("X-Slack-Signature")
        .ok()
        .flatten()
        .ok_or_else(|| Response::error("Missing X-Slack-Signature", 401))?;

    // 2. タイムスタンプのリプレイ攻撃チェック
    let ts: u64 = timestamp
        .parse()
        .map_err(|_| Response::error("Invalid timestamp", 401))?;

    let now_seconds = Date::now().as_millis() / 1000;
    let diff = if now_seconds > ts {
        now_seconds - ts
    } else {
        ts - now_seconds
    };
    if diff > TIMESTAMP_TOLERANCE_SECONDS {
        return Err(Response::error("Timestamp too old", 401));
    }

    // 3. リクエストボディを読み取る
    let body = req
        .text()
        .await
        .map_err(|_| Response::error("Failed to read body", 400))?;

    // 4. HMAC-SHA256で署名を計算
    let signing_secret = ctx
        .secret("SLACK_APP_SIGNING_SECRET")
        .map_err(|_| Response::error("Server configuration error", 500))?
        .to_string();

    let sig_basestring = format!("v0:{timestamp}:{body}");

    let mut mac = HmacSha256::new_from_slice(signing_secret.as_bytes())
        .map_err(|_| Response::error("Server configuration error", 500))?;
    mac.update(sig_basestring.as_bytes());

    // 5. 署名を定数時間で比較（タイミング攻撃防止）
    let expected_signature = signature
        .strip_prefix("v0=")
        .ok_or_else(|| Response::error("Invalid signature format", 401))?;

    let expected_bytes = hex::decode(expected_signature)
        .map_err(|_| Response::error("Invalid signature format", 401))?;

    mac.verify_slice(&expected_bytes)
        .map_err(|_| Response::error("Invalid signature", 401))?;

    Ok(body)
}
