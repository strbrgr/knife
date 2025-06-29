use ratatui::widgets::ListState;
use reqwest::{Client, Method, RequestBuilder, StatusCode};
use serde_json::Value;

use crate::components::list::{RepoItem, Repositories, RepositoryInfo, Status};

pub struct RepositoryClient {
    client: Client,
    token: String,
}

impl RepositoryClient {
    pub fn new(token: &str) -> Self {
        Self {
            client: Client::new(),
            token: token.to_owned(),
        }
    }

    fn build_request(&self, method: reqwest::Method, uri: &str) -> RequestBuilder {
        self.client
            .request(method, uri)
            .header("Accept", "application/vnd.github+json")
            .header("Authorization", format!("Bearer {}", self.token))
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("User-Agent", "knife")
    }

    pub async fn get_repository_data(
        &mut self,
    ) -> Result<Repositories, Box<dyn std::error::Error>> {
        let repo_owner = self.get_owner().await?;
        let repo_items = self.get_repos(&repo_owner).await?;

        Ok(Repositories {
            repo_owner,
            repo_items,
        })
    }

    async fn get_owner(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let url = "https://api.github.com/user";
        let res = self.build_request(Method::GET, url).send().await?;

        let body = res.text().await?;

        let value: Value = serde_json::from_str(&body)?;
        let repo_owner = value
            .get("login")
            .and_then(Value::as_str)
            .ok_or("Primary key not found")?;

        Ok(repo_owner.to_owned())
    }

    async fn get_repos(
        &mut self,
        repo_owner: &str,
    ) -> Result<RepositoryInfo, Box<dyn std::error::Error>> {
        let url = format!("https://api.github.com/users/{repo_owner}/repos");
        let res = self.build_request(Method::GET, &url).send().await?;

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

    pub async fn delete_repo(
        &self,
        repo_owner: &str,
        repo: &str,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!("https://api.github.com/repos/{repo_owner}/{repo}");
        let res = self.build_request(Method::DELETE, &url).send().await?;
        Ok(res.status())
    }
}
