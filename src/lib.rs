use project::Project;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::Duration;

pub mod archivist;
pub mod item;
pub mod project;
pub mod task;
pub mod background;

const TRACKER_NODES: [&str; 7] = [
    // "http://localhost:8080", // 测试环境
    "https://0.tracker.saveweb.org",
    "https://1.tracker.saveweb.org",
    "https://ipv4.1.tracker.saveweb.org",
    "https://ipv6.1.tracker.saveweb.org",
    // "https://2.tracker.saveweb.org", // 这台宕了
    "https://3.tracker.saveweb.org",
    "https://ipv4.3.tracker.saveweb.org",
    "https://ipv6.3.tracker.saveweb.org",
];

pub struct Tracker {
    api_base: Arc<RwLock<&'static str>>,
    api_version: &'static str,
    project_id: String,
    http_client: reqwest::Client,
    client_version: String,
    archivist: String,
    project: Option<Project>,
}

impl Tracker {
    pub fn new(
        project_id: String,
        client_version: String,
        archivist: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let api_base = Arc::new(RwLock::new(TRACKER_NODES[1]));
        Self::start_select_tracker_background(Arc::clone(&api_base));
        Ok(Tracker {
            api_base,
            api_version: "v1",
            project_id: project_id,
            http_client: reqwest::Client::builder()
                .timeout(Duration::from_secs(60))
                .build()?,
            client_version,
            archivist,
            project: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_tracker() {
        let mut tracker = Tracker::new("test".into(), "1.1".into(), "neko".into()).unwrap();
        let project = tracker.get_project().await;
        println!("{:?}", project);
        let task = tracker.claim_task(true).await.unwrap();

        println!("{:?}", task);
        let payload = r#"{"hhhh":123123, "f": 123.123}"#.to_string();

        let resp = tracker
            .insert_item(&task, String::from("DONE"), payload)
            .await;
        println!("{:?}", resp);
    }
}
