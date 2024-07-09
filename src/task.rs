use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};

use crate::{
    item::{Item, ItemStatusType},
    Tracker,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "TODO")]
    Todo,
    #[serde(rename = "PROCESSING")]
    Processing,
    #[serde(rename = "DONE")]
    Done,
    #[serde(rename = "FAIL")]
    Fail,
    /// 特殊: 任务冻结 （把一些反复失败的任务状态设成 FEZZ，防止反复 re-queue）
    #[serde(rename = "FEZZ")]
    Fezz,
} //每个项目的状态都可以自己定义
  // 只有 TODO PROCESSING 是必须的
  //草
impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Id {
    Int(i64),
    Str(String),
}
impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Id::Int(i) => write!(f, "{}", i),
            Id::Str(s) => write!(f, "{}", s),
        }
    }
}

/// MongoDB ObjectId
type ObjectId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub _id: ObjectId,
    pub id: Id,
    pub status: Status,
    pub archivist: String,
    pub claimed_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Tracker {
    pub async fn claim_task(&mut self, with_delay: bool) -> Option<Task> {
        if with_delay {
            let project = self.get_project().await;
            tokio::time::sleep(tokio::time::Duration::from_secs_f64(
                project.client.claim_task_delay,
            ))
            .await;
        }

        let api_base = *self.api_base.read().await;

        let url = format!(
            "{}/{}/project/{}/{}/{}/claim_task",
            api_base, self.api_version, self.project_id, self.client_version, self.archivist
        );
        println!("{}", url);
        let resp = self.http_client.post(&url).send().await.unwrap();
        return after_claim_task(resp).await;
    }

    pub async fn update_task(&self, task_id: Id, to_status: Status) -> String {
        let mut post_data = std::collections::HashMap::new();
        post_data.insert("status", to_status.to_string());
        post_data.insert("task_id_type", task_id.to_string());

        let api_base = *self.api_base.read().await;

        let url = format!(
            "{}/{}/{}/{}/{}/update_task/{}",
            api_base,
            self.api_version,
            self.project_id,
            self.client_version,
            self.archivist,
            task_id
        );
        let resp = self
            .http_client
            .post(&url)
            .form(&post_data)
            .send()
            .await
            .unwrap();
        return after_update_task(resp).await.unwrap();
    }

    pub async fn insert_many(&self, items: Vec<Item>) -> String {
        if items.is_empty() {
            return "len(Items) == 0, nothing to insert".to_string();
        }

        let api_base = *self.api_base.read().await;
        let url = format!(
            "{}/{}/project/{}/{}/{}/insert_many/{}",
            // TODO: 该找个 path builder 了？
            api_base,
            self.api_version,
            self.project_id,
            self.client_version,
            self.archivist,
            items.len()
        );

        let req = self
            .http_client
            .post(&url)
            .json(&items)
            .header(reqwest::header::ACCEPT, "*/*")
            .build()
            .unwrap();
        let resp = self.http_client.execute(req).await.unwrap();
        return after_insert_item(resp).await;
    }

    pub async fn insert_item(
        &self,
        task: &Task,
        item_status: String, // TODO
        payload: String,
    ) -> String {
        let api_base = *self.api_base.read().await;

        let url = format!(
            "{}/{}/project/{}/{}/{}/insert_item/{}",
            api_base,
            self.api_version,
            self.project_id,
            self.client_version,
            self.archivist,
            task.id
        );
        println!("{}", url);

        // type Item struct {
        //     Item_id          string `json:"item_id" binding:"required"`
        //     Item_id_type     string `json:"item_id_type" binding:"required,oneof=str int"`
        //     Item_status      string `json:"item_status" binding:"required"`
        //     Item_status_type string `json:"item_status_type" binding:"required,oneof=None str int"`
        //     Payload          string `json:"payload" binding:"required"`
        // }

        // 也可以发 HTTP Form
        let item = Item {
            item_id: task.id.to_string(),
            item_id_type: String::from("str"), /* (&task.id).into() */
            item_status: item_status.to_string(),
            item_status_type: ItemStatusType::Str,
            payload,
        };

        let req = self
            .http_client
            .post(&url)
            .json(&item)
            .header(reqwest::header::ACCEPT, "*/*")
            .header(reqwest::header::USER_AGENT, "rust-cat")
            .build()
            .unwrap();

        let resp = self.http_client.execute(req).await.unwrap();

        return after_insert_item(resp).await;
    }
}

async fn after_claim_task(r: Response) -> Option<Task> {
    let status = r.status();
    println!("status: {}", status);
    if status == 404 {
        return None;
    }
    if status == 200 {
        let task: Option<Task> = r.json().await.ok(); // 似乎是类型不正确
        println!("{:?}", task);
        return task;
    }

    let body = r.text().await.unwrap();
    panic!("{}", body);
}

async fn after_update_task(r: Response) -> Option<String> {
    let status = r.status();
    let body = r.text().await.ok()?;
    if status == 200 {
        Some(body)
    }
    /* if r.status() == 400 { panic!(body); } */
    else {
        panic!("{}: {}", status, body);
    }
}

async fn after_insert_item(r: Response) -> String {
    let status = r.status();
    let body = r.text().await.unwrap();
    if status == 200 {
        return body;
    }
    panic!("{}: {}", status, body);
}
