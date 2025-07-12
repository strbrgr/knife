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

        let repos = vec![
            Repository {
                name: "web-frontend".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "api-gateway".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "user-service".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "payment-processor".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "notification-service".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "legacy-auth".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "mobile-app".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "data-analytics".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "inventory-management".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "email-templates".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "database-migrations".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "search-engine".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "cache-service".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "log-aggregator".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "backup-scripts".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "testing-framework".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "deployment-tools".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "monitoring-dashboard".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "old-website".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "chat-service".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "file-storage".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "order-processing".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "customer-support".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "reporting-service".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "security-scanner".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "image-processor".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "recommendation-engine".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "beta-features".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "admin-panel".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "webhook-handler".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "social-media-integration".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "ml-training-pipeline".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "config-management".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "rate-limiter".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "prototype-v1".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "audit-logger".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "health-checker".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "queue-worker".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "legacy-db-connector".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "feature-flags".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "integration-tests".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "documentation-site".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "load-balancer".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "session-manager".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "experimental-ui".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "metrics-collector".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "theme-engine".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "old-mobile-app".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "content-management".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "api-documentation".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "web-frontend".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "api-gateway".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "user-service".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "payment-processor".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "notification-service".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "legacy-auth".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "mobile-app".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "data-analytics".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "inventory-management".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "email-templates".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "database-migrations".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "search-engine".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "cache-service".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "log-aggregator".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "backup-scripts".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "testing-framework".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "deployment-tools".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "monitoring-dashboard".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "old-website".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "chat-service".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "file-storage".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "order-processing".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "customer-support".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "reporting-service".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "security-scanner".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "image-processor".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "recommendation-engine".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "beta-features".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "admin-panel".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "webhook-handler".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "social-media-integration".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "ml-training-pipeline".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "config-management".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "rate-limiter".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "prototype-v1".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "audit-logger".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "health-checker".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "queue-worker".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "legacy-db-connector".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "feature-flags".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "integration-tests".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "documentation-site".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "load-balancer".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "session-manager".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "experimental-ui".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "metrics-collector".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "theme-engine".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "old-mobile-app".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "content-management".to_string(),
                status: Status::Selected,
            },
            Repository {
                name: "api-documentation".to_string(),
                status: Status::Selected,
            },
        ];

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
