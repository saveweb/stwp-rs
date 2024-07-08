use std::time::Duration;

use project::Project;

pub mod archivist;
pub mod item;
pub mod project;
pub mod task;

pub struct Tracker {
    api_base: &'static str,
    api_version: String,
    // ping_client: reqwest::Client, // TODO
    project_id: String,
    http_client: reqwest::Client,
    client_version: String,
    archivist: String,
    project: Option<Project>,
}

const TRACKER_NODES: [&str; 9] = [
    "http://localhost:8080", // 测试环境
    "https://0.tracker.saveweb.org",
    "https://1.tracker.saveweb.org",
    "https://ipv4.1.tracker.saveweb.org",
    "https://ipv6.1.tracker.saveweb.org",
    "https://2.tracker.saveweb.org", // 这台宕了
    "https://3.tracker.saveweb.org",
    "https://ipv4.3.tracker.saveweb.org",
    "https://ipv6.3.tracker.saveweb.org",
];

pub fn get_tracker(
    project_id: &str,
    client_version: &str,
    archivist: &str,
) -> Result<Tracker, Box<dyn std::error::Error>> {
    Ok(Tracker {
        api_base: TRACKER_NODES[2],
        api_version: "v1".into(),
        // ping_client: reqwest::Client::builder()
        //     .timeout(Duration::from_secs(10))
        //     .build()?,
        project_id: project_id.to_string(),
        http_client: reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()?,
        client_version: client_version.to_string(),
        archivist: archivist.to_string(),
        project: None,
    })
}

impl Tracker {
    fn start_select_tracker_background(&self) {
        // todo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_tracker() {
        let mut tracker = get_tracker("test", "1.1", "neko").unwrap();
        // 但是不知道不加 tokio decorator 会不会有问题
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
    // called `Result::unwrap()` on an `Err` value: Error("invalid type: integer `404`, expected struct Project", line: 1, column: 3)
    // can you see terminal?
    // yeap
    // 我看看后端
} // 是不是还少抄了什么
  // 写项目 调用的第一个应该是调哪个函数？

// 就是先 get_tracker() 然后用 tracker 对象 .get_project()
// 意思是 async 了个寂寞？
// 问题不大， get_tracker 不需要 async
