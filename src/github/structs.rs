#![allow(non_snake_case)]
use serde::Deserialize;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Owner {
    pub login: String,
    pub owner_type: OwnerType,
}

impl fmt::Display for Owner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{login} ({owner_type})",
            login = self.login,
            owner_type = self.owner_type
        )
    }
}

#[derive(Clone, Debug)]
pub enum OwnerType {
    User,
    Organization,
}

impl fmt::Display for OwnerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OwnerType::User => f.write_str("User"),
            OwnerType::Organization => f.write_str("Organization"),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Repository {
    pub id: i32,
    pub name: String,
    pub owner: OwnerForRepo,
}

impl fmt::Display for Repository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{owner}/{name}",
            owner = self.owner.login,
            name = self.name
        )
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct OwnerForRepo {
    pub login: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GitHubIssue {
    pub body: String,
    pub createdAt: String,
    pub number: i32,
    pub title: String,
    pub updatedAt: String,
    pub url: String,
}
