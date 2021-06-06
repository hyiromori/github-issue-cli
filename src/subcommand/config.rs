use crate::github::github_owners::get_github_owners;
use crate::github::github_repo::get_github_repos;
use crate::util::config::{get_config_file_path, write_config, Config};
use crate::util::select::select_in_menu;
use crate::zenhub::workspace::get_zenhub_workspaces;

pub async fn config(_args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let owners = get_github_owners().await?;
    let owner = select_in_menu(&String::from("Select repo owner"), &owners);
    match &owner {
        None => {
            panic!("Owner not found or unselected.")
        }
        Some(val) => {
            println!("Owner    : {owner_name}", owner_name = val.login);
        }
    }

    let repos = get_github_repos(&owner.unwrap()).await?;
    let repo = select_in_menu(&String::from("Select repo"), &repos);
    match &repo {
        None => {
            panic!("Repo not found or unselected.")
        }
        Some(val) => {
            println!(
                "Repo     : {repo_owner}/{repo_name}",
                repo_owner = val.owner.login,
                repo_name = val.name
            );
        }
    }

    let workspaces = get_zenhub_workspaces(&repo.unwrap().get_repo_id()).await?;
    let workspace = select_in_menu(&String::from("Select ZenHub workspace"), &workspaces);
    match workspace {
        None => {
            panic!("Workspace not found or unselected.")
        }
        Some(val) => {
            println!(
                "Workspace: {workspace_name} (ID: {workspace_id})",
                workspace_name = val.name,
                workspace_id = val.id
            );
            let _ = write_config(&Config {
                workspace_id: String::from(&val.id),
                workspace_name: String::from(&val.name),
            });
            println!("Config saved: {}", get_config_file_path());
        }
    };

    Ok(())
}
