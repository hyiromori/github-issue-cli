use crate::github::github_repo::get_github_repo_by_id;
use crate::util::config::Config;
use crate::util::select::select_in_menu;
use crate::zenhub::board::get_pipelines;
use crate::zenhub::structs::Board;

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

pub async fn board(config: &Config, _args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let actions = vec![BoardAction::Pipeline];
    let action = select_in_menu(&String::from("Choose action:"), &actions).unwrap();

    match action {
        BoardAction::Pipeline => {
            let repositories = &config.workspace.repositories;
            let repo = get_github_repo_by_id(repositories.first().unwrap()).await?;
            println!("{:#?}", repo);

            let pipelines = get_pipelines(&config.workspace.id, &repo.get_repo_id()).await?;
            let pipeline = select_in_menu(&String::from("Select pipeline"), &pipelines);
            println!("{:#?}", pipeline);
        }
    };

    Ok(())
}
