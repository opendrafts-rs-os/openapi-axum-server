use gloo_net::http::Request;

pub async fn testauth(api_base: &str, token: Option<String>) -> Result<String, String> {
    let url = format!("{}/testauth", api_base.trim_end_matches('/'));

    let mut req = Request::get(&url);

    if let Some(tok) = token {
        req = req.header("Authorization", &format!("Bearer {}", tok));
    }

    let resp = req.send().await.map_err(|e| e.to_string())?;

    if resp.ok() {
        let body = resp.text().await.unwrap_or_default();
        Ok(if body.is_empty() { "OK".into() } else { body })
    } else if resp.status() == 401 || resp.status() == 403 {
        Err("Unauthorized (401/403)".into())
    } else {
        let body = resp.text().await.unwrap_or_default();
        Err(format!("Err {}: {}", resp.status(), body))
    }
}