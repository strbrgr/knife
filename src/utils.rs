use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    login: String,
}

#[derive(Debug, Deserialize)]
pub struct Repo {
    name: String,
}

pub async fn get_user_with(token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let uri = "https://api.github.com/user".to_string();
    let res = Client::new()
        .get(uri)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {token}"))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "knife") // Required by GitHub API
        .send()
        .await?;

    let body = res.text().await?;
    let user: User = serde_json::from_str(&body)?;

    Ok(user.login)
}

pub async fn get_repos_with(
    user: &str,
    token: &str,
) -> Result<Vec<Repo>, Box<dyn std::error::Error>> {
    let uri = format!("https://api.github.com/users/{user}/repos").to_string();
    let res = Client::new()
        .get(uri)
        .header("Accept", "application/vnd.github+json")
        .header("Authorization", format!("Bearer {token}"))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "knife") // Required by GitHub API
        .send()
        .await?;

    let body = res.text().await?;
    let repos: Vec<Repo> = serde_json::from_str(body.as_str())?;

    Ok(repos)
}
