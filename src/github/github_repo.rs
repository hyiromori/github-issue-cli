#![allow(non_snake_case)]
use crate::github::github_api::{get_github_api_v3, request_github_graphql_api};
use crate::github::structs::{Owner, OwnerForRepo, OwnerType, Repository};
use base64::{decode, encode};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{Error, ErrorKind};

#[derive(Deserialize, Debug)]
pub struct UserResponse {
    data: UserData,
}

#[derive(Deserialize, Debug)]
pub struct UserData {
    user: UserOrOrganizationData,
}

#[derive(Deserialize, Debug)]
pub struct OrganizationResponse {
    data: OrganizationData,
}

#[derive(Deserialize, Debug)]
pub struct OrganizationData {
    organization: UserOrOrganizationData,
}

#[derive(Deserialize, Debug)]
pub struct UserOrOrganizationData {
    repositories: Repositories,
}

#[derive(Deserialize, Debug)]
pub struct Repositories {
    nodes: Vec<Repository>,
}

#[derive(Serialize, Clone, Debug)]
pub struct Variables {
    owner: String,
}

pub async fn get_github_repos(
    owner: &Owner,
) -> Result<Vec<Repository>, Box<dyn std::error::Error>> {
    let owner_type: String = match owner.owner_type {
        OwnerType::User => String::from("user"),
        OwnerType::Organization => String::from("organization"),
    };
    let query = format!(
        "query ($owner: String!) {{
          {owner_type} (login: $owner) {{
            repositories(first: 100, orderBy: {{ field: UPDATED_AT, direction: DESC }}){{
              nodes {{
                id
                name
                owner {{
                  login
                }}
              }}
            }}
          }}
        }}",
        owner_type = owner_type
    );
    let variables = Variables {
        owner: String::from(&owner.login),
    };
    let response = request_github_graphql_api(query, variables).await?;
    if response.status() != 200 {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "Failed get_github_repo_id",
        )));
    }
    // let data = response.text().await?;
    // println!("{:?}", data);
    // Ok(vec![])
    let data: Vec<Repository> = match owner.owner_type {
        OwnerType::User => {
            let data = response.json::<UserResponse>().await?;
            data.data.user.repositories.nodes
        }
        OwnerType::Organization => {
            let data = response.json::<OrganizationResponse>().await?;
            data.data.organization.repositories.nodes
        }
    };
    Ok(data)
}

#[derive(Deserialize)]
struct RepositoryForApiV3 {
    id: i32,
    name: String,
    owner: OwnerForRepo,
}

pub async fn get_github_repo_by_id(
    repo_id: &i32,
) -> Result<Repository, Box<dyn std::error::Error>> {
    let path = format!("/repositories/{repo_id}", repo_id = &repo_id);
    let res = get_github_api_v3(&path).await?;
    let temp_data = res.json::<RepositoryForApiV3>().await?;

    let data = Repository {
        id: encode(format!("000:Repository{id}", id = temp_data.id)),
        name: temp_data.name,
        owner: temp_data.owner,
    };
    Ok(data)
}
