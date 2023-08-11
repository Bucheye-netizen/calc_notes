use anyhow::{anyhow, Result};
use axum::http::{HeaderName, HeaderValue};
use once_cell::sync::Lazy;
use reqwest::Client;
use reqwest::{
    cookie::{Cookie, CookieStore, Jar},
    Response,
};
use serde_json::json;
use std::env;
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

    let cookies = COOKIE_JAR.cookies(&response.url());

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

static COOKIE_JAR: Lazy<Arc<Jar>> = Lazy::new(|| {
    Arc::new(Jar::default())
});

static BACKEND_URL: Lazy<String> = Lazy::new(|| {
    dotenvy::dotenv().ok();
    env::var("BACKEND_URL").expect("Set the BACKEND_URL environment variable")
});

/// An admin user for test purposes
static TEST_ADMIN: Lazy<(String, String)> = Lazy::new(|| {
    dotenvy::dotenv().ok();
    (
        env::var("TEST_ADMIN").expect("Set the BACKEND_URL environment variable"),
        env::var("TEST_ADMIN_PASSWORD").expect("Set TEST_ADMIN_PASSWORD the environment variable"),
    )
});

/// Tests whether the database can get notes
#[tokio::test]
async fn data() -> Result<()> {
    let client = Client::builder()
        .cookie_store(true)
        .cookie_provider(COOKIE_JAR.clone())
        .build()?;

    let response = client
        .get(format!("{}/data/notes/get/Limits", BACKEND_URL.as_str()))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!(fmt_response(response).await));
    }

    println!("{}", fmt_response(response).await);

    Ok(())
}

#[tokio::test]
async fn auth() -> Result<()> {
    let client = Client::builder()
        .cookie_store(true)
        .cookie_provider(COOKIE_JAR.clone())
        .build()?;

    let response = client
        .post(format!("{}/auth/login", BACKEND_URL.as_str()))
        .json(&json!([TEST_ADMIN.0, TEST_ADMIN.1]))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!(fmt_response(response).await));
    }

    println!("{}", fmt_response(response).await);

    let response = client
        .get(format!("{}/auth/status", BACKEND_URL.as_str()))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!(fmt_response(response).await));
    }

    println!("{}", fmt_response(response).await);

    let response = client
        .get(format!("{}/auth/logout", BACKEND_URL.as_str()))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!(fmt_response(response).await));
    }

    println!("{}", fmt_response(response).await);

    return Ok(());
}

/// Checks whether updating works.
/// TODO: Add check to ensure update occurred, not just that it didn't error.
#[tokio::test]
async fn patch() -> Result<()> {
    let client = Client::builder()
        .cookie_store(true)
        .cookie_provider(COOKIE_JAR.clone())
        .build()?;

    client
        .post(format!("{}/auth/login", BACKEND_URL.as_str()))
        .json(&json!([TEST_ADMIN.0, TEST_ADMIN.1]))
        .send()
        .await?;

    let response = client
        .patch(format!("{}/data/notes/patch", BACKEND_URL.as_str()))
        .json(&json!(
            {
                "set":
                {
                    "source": "Updated body",
                    "pub_date": -2000
                },
                "at":
                [
                    [["title", "=", "Test"], ""]
                ]
            }
        ))
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!(fmt_response(response).await));
    }

    println!("{}", fmt_response(response).await);

    let response = client
        .patch(format!("{}/data/notes/patch", BACKEND_URL.as_str()))
        .json(&json!(
            {
                "set":
                {
                    "source": "<h1>Usage</h1> <p>This is almost entirely for testing purposes</p>",
                    "pub_date": 0
                },
                "at":
                [
                    [["title", "=", "Test"], ""]
                ]
            }
        ))
        .send()
        .await?;

    /*
        [
            [[ "author", "=", "Lisan Kontra"], ""]
        ],
    */

    if !response.status().is_success() {
        return Err(anyhow!(fmt_response(response).await));
    }

    println!("{}", fmt_response(response).await);

    client
        .get(format!("{}/auth/logout", BACKEND_URL.as_str()))
        .json(&json!([TEST_ADMIN.0, TEST_ADMIN.1]))
        .send()
        .await?;

    return Ok(());
}
