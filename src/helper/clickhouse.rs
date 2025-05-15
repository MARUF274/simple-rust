use reqwest::{ redirect::Policy, ClientBuilder, StatusCode };
use serde_json::Value;
use dotenv::dotenv;
use std::env;
use serde_json::json;

use crate::helper::parser::to_str;

use super::io::get_env_or_default;

#[allow(dead_code)]
pub enum Query {
    Select(String),
    Insert(String),
}

#[allow(dead_code)]
pub async fn ch_query(key: &str, query: Query) -> Result<Value, String> {

    let keyid = get_env_or_default(&format!("CH_KEYID"), String::from("localhost"));
    let key_secret = get_env_or_default(&format!("CH_KEY_SECRET"), String::from("localhost"));
    let ch_url = get_env_or_default(&format!("CH_URL"), String::from("localhost"));


    match query {
        Query::Select(query) => {
            let url = format!(
                "{}format=JSON",ch_url 
            );

            let json_output = json!({
                "sql": query
            });


            let client = ClientBuilder::new()
                .cookie_store(true)
                .danger_accept_invalid_certs(true)
                .redirect(Policy::limited(20))
                .build()
                .map_err(|e| e.to_string())?;

            let res = client
                .post(url)
                .basic_auth(&keyid, Some(&key_secret))
                .header("Content-Type", "application/json")
                .body(json_output.to_string())
                .send().await
                .map_err(|e| e.to_string())?;

            if res.status() == StatusCode::OK {
                println!("========================== SUCESSSSS ==========================");
                let body = res.json::<Value>().await.map_err(|e| e.to_string())?;
                return Ok(body);
            }

            Err(res.status().to_string())
        }
        Query::Insert(data) => {
            let url = format!(
                "{}format=JSONEachRow", ch_url
            );

            println!("========================== url ==========================\n{url}");
            // println!("========================== data =========================\n{}", data  );

            let json_output = json!({
                "sql": data
            });

            println!("========================== data =========================\n{}", json_output);

            let client = ClientBuilder::new()
                .cookie_store(true)
                .danger_accept_invalid_certs(true)
                .redirect(Policy::limited(20))
                .build()
                .map_err(|e| e.to_string())?;

            let res = client
                .post(url)
                .header("Content-Type", "application/json")
                .basic_auth(&keyid, Some(&key_secret))
                .body(json_output.to_string())
                .send().await
                .map_err(|e| e.to_string())?;

            if res.status() == StatusCode::OK {
                return Ok(Value::Null);
            }
            let status = res.status().to_string();
            let body = res.text().await.map_err(|e| e.to_string())?;
            println!("body: {}", body);
            Err(status)
        }
    }
}
pub async fn conn(query_str: String) -> anyhow::Result<(Value, String)> {
    dotenv().ok();

    let host = env::var("CH_HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("CH_PORT").unwrap_or("8123".to_string());
    let username = env::var("CH_USER_NAME").unwrap_or("user_ancore".to_string());
    let password = env::var("CH_PASSWORD").unwrap_or("ANCore_Tsel1".to_string());

    let url = format!("http://{}:{}/?user={}&password={}&query=", host, port, username, password);

    // println!("{}",query_str);
    println!("{}", url);

    let client = ClientBuilder::new()
        .cookie_store(true)
        .danger_accept_invalid_certs(true)
        .redirect(Policy::limited(20))
        .build();

    let res = client?
        .post(url)
        .header("Content-Type", "application/json")
        .body(query_str)
        .send().await?;

    println!("{:?}", res);
    println!("Status: {}", res.status());

    let status = res.status().to_string();

    let data = res.json::<Value>().await?;

    Ok((data, status))
}
