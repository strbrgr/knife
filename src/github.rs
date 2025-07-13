use crate::ui::{GithubContent, Repository, Status};
use ratatui::widgets::ListState;
use reqwest::{Client, Error, Method, RequestBuilder, StatusCode};
use serde_json::Value;

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

    pub async fn get_owner(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        const USER_URL: &str = "https://api.github.com/user";
        let res = self.build_request(Method::GET, USER_URL).send().await?;
        if !res.status().is_success() {
            let error_msg = format!(
                "Could not get owner from GitHub. Request failed with status code: {}",
                res.status()
            );
            return Err(error_msg.into());
        }
        let body = res.text().await?;
        let value: Value = serde_json::from_str(&body)?;
        let owner = value
            .get("login")
            .and_then(Value::as_str)
            .ok_or("Primary key not found")?;

        Ok(owner.to_owned())
    }

    pub async fn get_repos(
        &mut self,
        owner: &str,
    ) -> Result<GithubContent, Box<dyn std::error::Error>> {
        let url = format!("https://api.github.com/users/{owner}/repos");
        let res = self.build_request(Method::GET, &url).send().await?;
        if !res.status().is_success() {
            let error_msg = format!(
                "Could not get repositories from GitHub. Request failed with status code: {}",
                res.status()
            );
            return Err(error_msg.into());
        }

        let body = res.text().await?;

        let value: Value = serde_json::from_str(&body)?;

        let repos = value
            .as_array()
            .ok_or("Expected an array")?
            .iter()
            .filter_map(|item| item.get("name").and_then(Value::as_str))
            .map(|s| Repository {
                name: s.to_owned(),
                status: Status::Selected,
            })
            .collect::<Vec<_>>();

        Ok(GithubContent {
            repos,
            owner: owner.to_owned(),
            list_state: ListState::default(),
        })
    }

    pub async fn delete_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<StatusCode, Box<dyn std::error::Error>> {
        let url = format!("https://api.github.com/repos/{owner}/{repo}");
        let res = self.build_request(Method::DELETE, &url).send().await?;
        Ok(res.status())
    }
}
