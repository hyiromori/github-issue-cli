use crate::zenhub::structs::{EpicIssue, ZenHubIssue};
use crate::zenhub::zenhub_api::{get_zenhub_api, post_zenhub_api};
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

#[derive(Deserialize, Debug)]
struct Response {
    epic_issues: Vec<EpicIssue>,
}

pub async fn get_epic_issues(
    repo_id: &String,
) -> Result<Vec<EpicIssue>, Box<dyn std::error::Error>> {
    // https://github.com/ZenHubIO/API#get-epics-for-a-repository
    let path = format!("/p1/repositories/{repo_id}/epics", repo_id = repo_id);
    let response = get_zenhub_api(&path).await?;
    if response.status() == 200 {
        let data = response.json::<Response>().await?;
        Ok(data.epic_issues)
        // println!("{:#?}", response.text().await?);
        // Ok(vec![] as Vec<EpicIssue>)
    } else {
        Err(Box::new(Error::new(
            ErrorKind::Other,
            "Failed move_pipeline",
        )))
    }
}
