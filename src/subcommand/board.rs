use crate::github::github_repo::get_github_repo_by_id;
use crate::github::structs::Repository;
use crate::util::config::Config;
use crate::util::select::select_in_menu;
use crate::zenhub::board::get_pipelines;
use crate::zenhub::structs::{Pipeline, EpicIssue, ZenHubIssueForPipeline};

#[derive(Debug, Clone)]
enum BoardAction {
    Pipeline,
}

impl ToString for BoardAction {
    fn to_string(&self) -> String {
        match self {
            BoardAction::Pipeline => String::from("Pipeline"),
        }
    }
}

#[derive(Debug)]
struct RepoAndPipelines {
    repo: Repository,
    pipelines: Vec<Pipeline>,
}

pub async fn board(config: &Config, _args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let actions = vec![BoardAction::Pipeline];
    let action = select_in_menu(&String::from("Choose action:"), &actions).unwrap();

    match action {
        BoardAction::Pipeline => {
            let repo_ids = &config.workspace.repositories;

            // TODO: Parallel request
            let mut list: Vec<RepoAndPipelines> = vec![];
            for repo_id in repo_ids {
                let repo = get_github_repo_by_id(&repo_id).await?;
                let pipelines = get_pipelines(&config.workspace.id, &repo_id).await?;
                list.push(RepoAndPipelines { repo, pipelines });
            }

            let repo_and_pipelines = list.first().unwrap();
            let pipeline = select_in_menu(
                &String::from("Select pipeline"),
                &repo_and_pipelines.pipelines,
            );
            if pipeline.is_none() {
                panic!("Owner not found or unselected.")
            }
            let pipeline = pipeline.unwrap();
            println!("{:#?}", pipeline.id);

            // let pipeline_issues: Vec<ZenHubIssueForPipeline> = vec![];
        }
    };

    Ok(())
}
