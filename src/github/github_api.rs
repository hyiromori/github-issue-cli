#![allow(non_snake_case)]
use reqwest::Response;
use serde::Serialize;
use std::env;

fn get_authorization_header_for_github() -> String {
    match env::var("GITHUB_ACCESS_TOKEN") {
        Ok(val) => format!("bearer {}", val),
        Err(_err) => ("").to_string(),
    }
}

#[derive(Serialize, Debug)]
pub struct GraphQlRequest<T: Serialize> {
    query: String,
    variables: T,
}

pub async fn request_github_graphql_api<T: Serialize>(
    query: String,
    variables: T,
) -> Result<Response, Box<dyn std::error::Error>> {
    let url = "https://api.github.com/graphql";
    let req = GraphQlRequest { query, variables };
    let body: String = serde_json::to_string(&req).unwrap();

    let res = reqwest::Client::new()
        .post(url)
        .header("User-Agent", "mryhryki/github-issue-cli")
        .header("Authorization", get_authorization_header_for_github())
        .body(body)
        .send()
        .await?;
    if res.status() != 200 {
        println!("{:#?}", res.status());
    }

    Ok(res)
}

// https://developer.github.com/v3/
pub async fn get_github_api_v3(path: &String) -> Result<Response, Box<dyn std::error::Error>> {
    let url = format!(
        "{base}{path}",
        base = "https://api.github.com",
        path = &path
    );

    let res = reqwest::Client::new()
        .get(url)
        .header("User-Agent", "mryhryki/github-issue-cli")
        .header("Accept", "application/vnd.github.v3+json")
        .header("Authorization", get_authorization_header_for_github())
        .send()
        .await?;
    if res.status() != 200 {
        println!("{:#?}", res.status());
    }

    Ok(res)
}
