use std::time::Duration;

pub mod item;
pub mod task;
pub mod archivist;
pub mod project;

pub struct Tracker {
    api_base: &'static str,
    api_version: String,
    ping_client: reqwest::Client,
    project_id: String,
    http_client: reqwest::Client,
    client_version: String,
    archivist: String,
}

const TRACKER_NODES: [&str; 9]  = [
    "http://localhost:8080/", // 测试环境
    "https://0.tracker.saveweb.org/",
    "https://1.tracker.saveweb.org/",
    "https://ipv4.1.tracker.saveweb.org/",
    "https://ipv6.1.tracker.saveweb.org/",
    "https://2.tracker.saveweb.org/", // 这台宕了
    "https://3.tracker.saveweb.org/",
    "https://ipv4.3.tracker.saveweb.org/",
    "https://ipv6.3.tracker.saveweb.org/",
];

/*
func GetTracker(project_id string, client_version string, archivist string) *Tracker {
	t := &Tracker{
		API_VERSION: "v1",
		PING_client: &http.Client{
			Timeout: 10 * time.Second,
		},
		project_id: project_id,
		HTTP_client: &http.Client{
			Timeout: 120 * time.Second,
		},
		client_version: client_version,
		archivist:      archivist,
		__gzPool: sync.Pool{
			New: func() interface{} {
				gz, err := gzip.NewWriterLevel(nil, gzip.BestCompression)
				if err != nil {
					panic(err)
				}
				return gz
			},
		},
	}
	return t
}
*/
#[tokio::main]
pub async fn get_tracker(project_id: &str, client_version: &str, archivist: &str) -> Result<Tracker, Box<dyn std::error::Error>> {

    Ok(
    Tracker {
        api_base: TRACKER_NODES[2],
        api_version: "v1".into(),
        ping_client: reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?,
        project_id: project_id.to_string(),
        http_client: reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()?,
        client_version: client_version.to_string(),
        archivist: archivist.to_string(),
    })
}

impl Tracker {
    fn start_select_tracker_background(&self) {
        // todo
    }
}