use crate::zenhub::structs::Pipeline;
use crate::zenhub::zenhub_api::get_zenhub_api;
use serde::Deserialize;
use std::io::{Error, ErrorKind};

#[derive(Deserialize, Debug)]
struct ResponseData {
    pipelines: Vec<Pipeline>,
}

pub async fn get_pipelines(
    workspace_id: &String,
    repo_id: &i32,
) -> Result<Vec<Pipeline>, Box<dyn std::error::Error>> {
    // https://github.com/ZenHubIO/API#get-a-zenhub-board-for-a-repository
    let path = format!(
        "/p2/workspaces/{workspace_id}/repositories/{repo_id}/board",
        workspace_id = workspace_id,
        repo_id = repo_id
    );
    let response = get_zenhub_api(&path).await?;
    if response.status() == 200 {
        let data: ResponseData = response.json::<ResponseData>().await?;
        Ok(data.pipelines)
    } else {
        Err(Box::new(Error::new(
            ErrorKind::Other,
            "Failed move_pipeline",
        )))
    }
}
