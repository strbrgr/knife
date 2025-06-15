use reqwest::Client;
use serde_json::Value;

use crate::components::list::{RepoItem, Status};

// What we are fetching
enum GithubResource {
    UserName,
    RepoNames,
}

enum GithubResponse {
    UserNames(Vec<String>),
    RepoItems(Vec<RepoItem>),
}

fn extract_username(response: &GithubResponse) -> Result<&String, Box<dyn std::error::Error>> {
    if let GithubResponse::UserNames(usernames) = response {
        usernames.first().ok_or_else(|| "No username found".into())
    } else {
        Err("Expected UserNames variant".into())
    }
}

pub async fn get_repos_from_github(
    token: &str,
) -> Result<Vec<RepoItem>, Box<dyn std::error::Error>> {
    let user_response = get_user(token).await?;
    let user_login = extract_username(&user_response)?;
    let repos_response = get_repos(user_login, token).await?;

    if let GithubResponse::RepoItems(items) = repos_response {
        Ok(items)
    } else {
        Err("Expected RepoItems variant".into())
    }
}

async fn get_user(token: &str) -> Result<GithubResponse, Box<dyn std::error::Error>> {
    let user_uri = "https://api.github.com/user";
    get_resource(user_uri, GithubResource::UserName, token).await
}

async fn get_repos(
    user_login: &str,
    token: &str,
) -> Result<GithubResponse, Box<dyn std::error::Error>> {
    let repos_uri = format!("https://api.github.com/users/{user_login}/repos");
    get_resource(&repos_uri, GithubResource::RepoNames, token).await
}

async fn get_resource(
    uri: &str,
    resource: GithubResource,
    token: &str,
) -> Result<GithubResponse, Box<dyn std::error::Error>> {
    let res = Client::new()
        .get(uri)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {token}"))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "knife")
        .send()
        .await?;

    let body = res.text().await?;

    let value: Value = serde_json::from_str(&body)?;
    match resource {
        GithubResource::UserName => {
            let primary_value = value
                .get("login")
                .and_then(Value::as_str)
                .ok_or("Primary key not found")?;
            Ok(GithubResponse::UserNames(vec![primary_value.to_string()]))
        }
        GithubResource::RepoNames => {
            let names = value
                .as_array()
                .ok_or("Expected an array")?
                .iter()
                .filter_map(|item| item.get("name").and_then(Value::as_str))
                .map(|s| RepoItem {
                    repo: s.to_owned(),
                    status: Status::Unselected,
                })
                .collect::<Vec<_>>();
            Ok(GithubResponse::RepoItems(names))
        }
    }
}
