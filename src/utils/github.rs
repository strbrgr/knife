use ratatui::widgets::ListState;
use reqwest::{Client, StatusCode};
use serde_json::Value;

use crate::components::list::{RepoItem, Repositories, RepositoryInfo, Status};

pub async fn get_data_from_github(token: &str) -> Result<Repositories, Box<dyn std::error::Error>> {
    let owner = get_user(token).await?;
    let repo_items = get_repos(&owner, token).await?;

    Ok(Repositories { owner, repo_items })
}

pub async fn delete_repo(
    owner: &str,
    repo: &str,
    token: &str,
) -> Result<StatusCode, Box<dyn std::error::Error>> {
    let delete_uri = format!("https://api.github.com/repos/{owner}/{repo}");

    let res = Client::new()
        .delete(delete_uri)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {token}"))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "knife")
        .send()
        .await?;

    Ok(res.status())
}

pub async fn get_user(token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let user_uri = "https://api.github.com/user";
    let res = Client::new()
        .get(user_uri)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {token}"))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "knife")
        .send()
        .await?;

    let body = res.text().await?;

    let value: Value = serde_json::from_str(&body)?;
    let primary_value = value
        .get("login")
        .and_then(Value::as_str)
        .ok_or("Primary key not found")?;

    Ok(primary_value.to_owned())
}

pub async fn get_repos(
    user_login: &str,
    token: &str,
) -> Result<RepositoryInfo, Box<dyn std::error::Error>> {
    let repos_uri = format!("https://api.github.com/users/{user_login}/repos");
    let res = Client::new()
        .get(repos_uri)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {token}"))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "knife")
        .send()
        .await?;

    let body = res.text().await?;

    let value: Value = serde_json::from_str(&body)?;

    let repos = value
        .as_array()
        .ok_or("Expected an array")?
        .iter()
        .filter_map(|item| item.get("name").and_then(Value::as_str))
        .map(|s| RepoItem {
            name: s.to_owned(),
            status: Status::Unselected,
        })
        .collect::<Vec<_>>();
    Ok(RepositoryInfo {
        repos,
        list_state: ListState::default(),
    })
}
