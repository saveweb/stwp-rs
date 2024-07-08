use serde::Deserialize;

use crate::Tracker;

#[derive(Debug, Deserialize)]
pub struct ProjectMeta {
    identifier: String,
    slug: String,
    icon: String,
    deadline: String,
}

#[derive(Debug, Deserialize)]
pub struct ProjectStatus {
    public: bool,
    paused: bool,
}

#[derive(Debug, Deserialize)]
pub struct ProjectClient {
    version: String,
    claim_task_delay: f64, // 用来做 QoS 的
}

#[derive(Debug, Deserialize)]
pub struct ProjectMongodb {
    db_name: String,
    item_collection: String,
    queue_collection: String,
    custom_doc_id_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    meta: ProjectMeta,
    status: ProjectStatus,
    client: ProjectClient,
    mongodb: ProjectMongodb,
}

impl Tracker {
    pub async fn fetch_project(&self) -> Result<Project, Box<dyn std::error::Error>> {
        println!("fetch_project... {}", self.project_id);
        let url = format!(
            "{}/{}/project/{}",
            self.api_base, self.api_version, self.project_id
        );
        let res = self.http_client.post(&url).send().await?;
        // parse response as json
        let project: Project = serde_json::from_str(&res.text().await?)?;
        Ok(project)
    }

    pub async fn get_project(&mut self) -> &Project {
        if self.project.is_none() {
            self.project = Some(self.fetch_project().await.unwrap());
        }
        self.project.as_ref().unwrap()
    } // if let 会转移所有权
}
