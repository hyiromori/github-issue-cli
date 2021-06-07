use crate::github::github_owners::get_github_owners;
use crate::github::github_repo::get_github_repos;
use crate::util::config::{get_config_file_path, write_config, Config};
use crate::util::select::select_in_menu;
use crate::zenhub::workspace::get_zenhub_workspaces;

pub async fn config(_args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let owners = get_github_owners().await?;
    let owner = select_in_menu(&String::from("Select repo owner"), &owners);
    if owner.is_none() {
        panic!("Owner not found or unselected.")
    }
    let owner = owner.unwrap();

    let repos = get_github_repos(&owner).await?;
    let repo = select_in_menu(&String::from("Select repo"), &repos);
    if repo.is_none() {
        panic!("Repo not found or unselected.")
    }
    let repo = repo.unwrap();

    let workspaces = get_zenhub_workspaces(&repo.id).await?;
    let workspace = select_in_menu(&String::from("Select ZenHub workspace"), &workspaces);
    if workspace.is_none() {
        panic!("Workspace not found or unselected.")
    }
    let workspace = workspace.unwrap();

    let _ = write_config(&Config { workspace });
    println!("Config saved: {}", get_config_file_path());

    Ok(())
}
