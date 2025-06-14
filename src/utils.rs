use reqwest::Client;
use serde_json::Value;

enum GithubResource {
    UserName,
    RepoNames,
}

pub async fn get_repos_from_github(token: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let user_uri = "https://api.github.com/user";
    let user = get_resource(user_uri, GithubResource::UserName, token).await?;

    let user_login = user.first().unwrap();
    let repos_uri = format!("https://api.github.com/users/{user_login}/repos");
    let repos = get_resource(&repos_uri, GithubResource::RepoNames, token).await?;
    Ok(repos)
}

async fn get_resource(
    uri: &str,
    resource: GithubResource,
    token: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
        GithubResource::RepoNames => {
            let names = value
                .as_array()
                .ok_or("Expected an array")?
                .iter()
                .filter_map(|item| item.get("name").and_then(Value::as_str))
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            Ok(names)
        }
        GithubResource::UserName => {
            let primary_value = value
                .get("login")
                .and_then(Value::as_str)
                .ok_or("Primary key not found")?;
            Ok(vec![primary_value.to_string()])
        }
    }
}
