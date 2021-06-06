use crate::github::github_api::request_github_graphql_api;
use crate::github::structs::{Owner, OwnerType};
use serde::{Deserialize};
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
    nodes: Vec<Organization>,
}

#[derive(Deserialize, Debug)]
struct Organization {
    login: String,
}
pub async fn get_github_owners() -> Result<Vec<Owner>, Box<dyn std::error::Error>> {
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
        return Err(Box::new(Error::new(ErrorKind::Other, "Failed get_owners")));
    }

    let mut owners = vec![];
    let data = response.json::<ResponseRoot>().await?;
    owners.push(Owner {
        login: data.data.viewer.login,
        owner_type: OwnerType::User,
    });
    for org in data.data.viewer.organizations.nodes {
        owners.push(Owner {
            login: org.login,
            owner_type: OwnerType::Organization,
        });
    }

    Ok(owners)
}
