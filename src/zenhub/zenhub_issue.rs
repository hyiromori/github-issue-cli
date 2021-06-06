use crate::zenhub::structs::ZenHubIssue;
use crate::zenhub::zenhub_api::{get_zenhub_api, post_zenhub_api};
use serde::Serialize;
use std::io::{Error, ErrorKind};

pub async fn get_zenhub_issue(
    repo_id: &String,
    issue_number: &i32,
) -> Result<ZenHubIssue, Box<dyn std::error::Error>> {
    // https://github.com/ZenHubIO/API#get-zenhub-workspaces-for-a-repository
    let path = format!(
        "/p1/repositories/{repo_id}/issues/{issue_number}",
        repo_id = repo_id,
        issue_number = issue_number
    );
    let response = get_zenhub_api(&path).await?;
    if response.status() == 200 {
        Ok(response.json::<ZenHubIssue>().await?)
    } else {
        Err(Box::new(Error::new(
            ErrorKind::Other,
            "Failed move_pipeline",
        )))
    }
}

#[derive(Serialize, Debug)]
struct MovePipelineBody {
    pipeline_id: String,
    position: String,
}

pub async fn move_pipeline(
    workspace_id: &String,
    repo_id: &String,
    issue_number: &i32,
    pipeline_id: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    // https://github.com/ZenHubIO/API#move-an-issue-between-pipelines
    let path = format!(
        "/p2/workspaces/{workspace_id}/repositories/{repo_id}/issues/{issue_number}/moves",
        workspace_id = workspace_id,
        repo_id = repo_id,
        issue_number = issue_number
    );
    let body = MovePipelineBody {
        pipeline_id: String::from(pipeline_id),
        position: String::from("bottom"),
    };
    let response = post_zenhub_api(&path, body).await?;
    if response.status() == 200 {
        Ok(())
    } else {
        Err(Box::new(Error::new(
            ErrorKind::Other,
            "Failed move_pipeline",
        )))
    }
}
