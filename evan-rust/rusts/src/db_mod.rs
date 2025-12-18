#![allow(dead_code)]
#![allow(unused_variables)]


//use reqwest::header;
use reqwest::{Client, header::AUTHORIZATION};
use serde::{Deserialize, Serialize};
use serde_json::json;


#[derive(Serialize)]
struct QueryRequest<'a> {
    query: &'a str,
}

// #[derive(Debug, Deserialize)]
// struct ApiResponse {
//     // Use serde_json::Value if you don't know the shape
//     data: serde_json::Value,
// }

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub success: bool,
    pub meta: Meta,
    pub results: Vec<User>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Meta {
    pub served_by: String,
    pub served_by_region: String,
    pub served_by_primary: bool,
    pub timings: Timings,

    pub duration: f64,
    pub changes: i64,
    pub last_row_id: i64,
    pub changed_db: bool,
    pub size_after: i64,
    pub rows_read: i64,
    pub rows_written: i64,
    pub total_attempts: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Timings {
    pub sql_duration_ms: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
}

//use reqwest::Client;

pub async fn query_db() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // ✅ No unwrap (avoid panic)
    let token = std::env::var("API_TOKEN")?;

    let response = client
        .post("https://d1-rest.dangolevan.workers.dev/query")
        .bearer_auth(token)
        .json(&json!({
            "query": "select * from users"
        }))
        .send()
        .await?
        .error_for_status()?;

    // ✅ Read body ONCE
    let body = response.text().await?;

    // ✅ 1. Print raw JSON
    println!("Raw JSON:\n{}", body);

    // ✅ 2. Deserialize full response
    let api: ApiResponse = serde_json::from_str(&body)?;

    // ✅ 3. Use typed data
    println!("Success: {}", api.success);
    println!("Rows read: {}", api.meta.rows_read);
    println!("SQL time: {} ms", api.meta.timings.sql_duration_ms);

    println!("served_by           : {}", api.meta.served_by);
    println!("served_by_region    : {}", api.meta.served_by_region);
    println!("served_by_primary   : {}", api.meta.served_by_primary);
    println!("duration (ms)       : {}", api.meta.duration);
    println!("changes             : {}", api.meta.changes);
    println!("last_row_id         : {}", api.meta.last_row_id);
    println!("changed_db          : {}", api.meta.changed_db);
    println!("size_after          : {}", api.meta.size_after);
    println!("rows_read           : {}", api.meta.rows_read);
    println!("rows_written        : {}", api.meta.rows_written);
    println!("total_attempts      : {}", api.meta.total_attempts);
    println!("sql_duration_ms     : {}", api.meta.timings.sql_duration_ms);

    println!("-----------------------------");
    let meta_json = serde_json::to_value(&api.meta)?;

    if let serde_json::Value::Object(map) = meta_json {
        for (key, value) in map {
            println!("{:<20} {}", key, value);
        }
    }
    println!("results:");

    for user in &api.results {
        println!(
            "{} | {} | {} | {}",
            user.user_id, user.name, user.email, user.age
        );
    }

    Ok(())
}

pub fn run() {
    // create a Tokio runtime ONLY here
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    // properly execute async code
    rt.block_on(async {
        if let Err(e) = query_db().await {
            eprintln!("DB error: {}", e);
        }
    });
}


pub async fn run1() {
    dotenvy::dotenv().ok();

    let client = Client::new();
    let token = std::env::var("API_TOKEN");
    let borrow_token = &token.unwrap();
    println!("{:?}", borrow_token);
    let res = client
        .post("https://d1-rest.dangolevan.workers.dev/query")
        .header(AUTHORIZATION, format!("Bearer {}", borrow_token))
        .json(&json!({
            "query": "select * from users"
        }))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{}", res);
}
