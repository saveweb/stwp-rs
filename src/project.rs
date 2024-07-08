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
    // 我写的是先同步获取一次 project ，然后后台每一分钟获取一次，然后超过几分钟没有正常拿到 project，就 panic
    // 我先不管后台的，跑起来再说
    // 草
    // 中肯的
    // pub async fn project() TODO
    pub async fn fetch_project(&self) -> Result<Project, Box<dyn std::error::Error>> {
        println!("fetch_project... {}", self.project_id);
        // curl -X POST https://0.tracker.saveweb.org/v1/project/test
        let url = format!("{}/project/{}", self.api_base, self.project_id);
        let res = self.http_client.post(&url).send().await?;
        // parse response as json
        let project: Project = serde_json::from_str(&res.text().await?)?;
        Ok(project)
    }
}

/* 
package savewebtracker

func (t *Tracker) Project() (proj Project) {
	if time.Since(t.__project_last_fetched) <= 3*time.Minute {
		return *t.__project
	}

	t.StartFetchProjectBackground()

	for t.__project == nil { // initial fetch
		time.Sleep(1 * time.Second)
		if t.__project != nil { // fetch success
			return t.Project()
		}
	}

	for { // not nil, but outdated
		if time.Since(t.__project_last_fetched) > 5*time.Minute { // over 5 minutes, abort
			panic("all fetch failed for 5 minutes")
		}
		if time.Since(t.__project_last_fetched) <= 3*time.Minute { // not outdated anymore
			return *t.__project
		}
		go t.FetchProject(5 * time.Second) // short timeout
		time.Sleep(8 * time.Second)
	}
}

func (t *Tracker) StartFetchProjectBackground() *Tracker {
	if t.__background_fetch_proj {
		return t
	}
	t.__background_fetch_proj = true
	go func() {
		for {
			go t.FetchProject(20 * time.Second)
			time.Sleep(1 * time.Minute)
		}
	}()
	return t
}

func (t *Tracker) FetchProject(timeout time.Duration) (proj *Project, err error) {
	fmt.Println("[client->tracker] fetch_project... ", t.project_id)

	ctx, cancel := context.WithTimeout(context.TODO(), timeout)
	time.AfterFunc(timeout, func() {
		cancel()
	})

	req, err := http.NewRequestWithContext(ctx, "POST", t.API_BASE+t.API_VERSION+"/project/"+t.project_id, nil)
	if err != nil {
		log.Print(err)
		return nil, err
	}
	r, err := t.HTTP_client.Do(req)
	if err != nil {
		log.Print(err)
		return nil, err
	}
	defer r.Body.Close()
	if r.StatusCode != 200 {
		return nil, errors.New("status code not 200")
	}
	proj = &Project{}
	err = json.NewDecoder(r.Body).Decode(proj)
	if err != nil {
		return nil, err
	}
	t.__project = proj
	t.__project_last_fetched = time.Now()
	fmt.Println("[client<-tracker] fetch_project. ", t.project_id)
	return proj, nil
}

 */
