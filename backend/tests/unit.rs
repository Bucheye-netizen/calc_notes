use anyhow::Result;
use axum::http::{HeaderName, HeaderValue};
use lazy_static::lazy_static;
use reqwest::{
    cookie::{Cookie, CookieStore, Jar},
    Response,
};
use serde_json::json;
use std::sync::Arc;

async fn fmt_response(response: Response) -> String {
    let mut output: String = String::new();

    output.push_str("=========REQUEST=========\n");
    output.push_str(&format!("RESPONSE URL:\n   {}\n", response.url()));
    output.push_str(&format!("STATUS:\n   {}\n", response.status()));
    output.push_str(&format!(
        "HEADERS:\n{}",
        response
            .headers()
            .iter()
            .map(|x: (&HeaderName, &HeaderValue)| {
                format!(
                    "   {}: {}\n",
                    x.0.as_str(),
                    x.1.to_str().unwrap_or("Header in raw bytes")
                )
            })
            .collect::<String>()
    ));
    let response_cookies = response.cookies().collect::<Vec<Cookie>>();
    if response_cookies.len() != 0 {
        output.push_str(&format!(
            "RESPONSE COOKIES:\n{}",
            response_cookies
                .into_iter()
                .map(|x: Cookie| { format!("   {}\n", x.value()) })
                .collect::<String>()
        ));
    }

    let cookies = cookie_jar().cookies(&response.url());

    if cookies.is_some() {
        output.push_str(&format!(
            "CLIENT COOKIES:\n    {}\n",
            cookies
                .unwrap()
                .to_str()
                .unwrap_or("Client cookies in non-ASCII format!")
        ));
    }

    output.push_str(&format!(
        "BODY:\n{}",
        response
            .text()
            .await
            .unwrap_or("Failed to query body!".to_string())
            .split("\n")
            .map(|x: &str| format!("   {}\n", x))
            .collect::<String>()
    ));

    output
}

lazy_static! {
    static ref COOKIE_JAR: Arc<Jar> = Arc::new(Jar::default());
}

fn cookie_jar() -> Arc<Jar> {
    lazy_static::initialize(&COOKIE_JAR);
    return COOKIE_JAR.clone();
}

/// Tests whether the database can get notes
#[tokio::test]
async fn get_note_test() -> Result<()> {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .cookie_provider(cookie_jar().clone())
        .build()?;
    let url = "http://localhost:3000";

    let response = client
        .get(format!("{}/api/data/notes/get/Limits", url))
        .send()
        .await?;

    println!("{}", fmt_response(response).await);

    Ok(())
}
