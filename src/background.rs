use std::sync::Arc;

use tokio::{sync::RwLock, time::Duration};

use crate::{project::Project, Tracker, TRACKER_NODES};

impl Tracker {
    pub fn select_tracker_background(api_base: Arc<RwLock<&'static str>>) {
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(60)).await;
                let mut write_guard: tokio::sync::RwLockWriteGuard<&str> = api_base.write().await;
                use futures::future::JoinAll;
                println!("Selecting best tracker...");
                let durations = TRACKER_NODES
                    .iter()
                    .map(|&node| Self::get_ping(node))
                    .collect::<JoinAll<_>>()
                    .await;
                let best_node = durations
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, &elapsed)| elapsed)
                    .map(|(idx, _)| TRACKER_NODES[idx])
                    .unwrap();
                *write_guard = best_node;
            }
        });
    }

    async fn get_ping(node: &str) -> Duration {
        let url = format!("{}/ping", node);
        let time = std::time::Instant::now();
        let resp = reqwest::get(&url).await.unwrap().text().await.unwrap();
        println!("ping {} got {}, elapsed: {:?}", node, resp, time.elapsed());
        time.elapsed()
    }
}
