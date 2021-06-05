#![allow(non_snake_case)]
use crate::github::github_api::request_github_graphql_api;
use base64::decode;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

#[derive(Deserialize, Debug)]
pub struct ResponseRoot {
    data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    viewer: Viewer,
}

#[derive(Deserialize, Debug)]
pub struct Viewer {
    login: String,
    organizations: Organizations,
}

#[derive(Deserialize, Debug)]
struct Organizations {
    nodes: Vec<OrganizationNode>,
}

#[derive(Deserialize, Debug)]
struct OrganizationNode {
    login: String
}

pub async fn get_owners() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let query = String::from(
        "query {
           viewer {
             login
             organizations(last: 100) {
               nodes {
                 login
               }
             }
           }
         }",
    );

    let response = request_github_graphql_api(query, ()).await?;
    if response.status() != 200 {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "Failed get_owners",
        )));
    }

    let mut owners = vec![];
    let data = response.json::<ResponseRoot>().await?;
    owners.push(data.data.viewer.login);
    for org in data.data.viewer.organizations.nodes {
        owners.push(org.login);
    }

    Ok(owners)
}
