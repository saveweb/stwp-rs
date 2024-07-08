use chrono::TimeZone;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display};

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
    /// 特殊: 任务冻结 （把一些  的状态设成 FEZZ，防止反复 re-queue）
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

pub type ObjectID = String;

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    obj_id: ObjectID,
    id: Id,
    status: Status,
    archivist: String,
    claimed_at: Option<String>,
    updated_at: Option<String>,
}

// pub fn get_datetime(d: String) -> chrono::DateTime<chrono::Utc> {
// 2024-06-06T15:45:00.008Z
// let t = TimeZone::from
// return t;
// }

impl Tracker {
    pub async fn claim_task(with_delay: bool) -> Option<Task> {
        if with_delay {
            tokio::time::sleep(tokio::time::Duration::from_secs(1) /* TODO */).await;
        }
        let resp = reqwest::get("https://www.rust-lang.org").await?;
        return Task::after_claim_task(resp).await;
    }
    async fn after_claim_task(r: reqwest::Response) -> Option<Task> {
        if r.status() == 404 {
            return None;
        }
        if r.status() == 200 {
            let task: Option<Task> = r.json().await.ok();
            return task;
        }

        let body = r.text().await.unwrap();
        panic!("{}", body);
    }

    pub async fn update_task(&self, task_id: Id, to_status: Status) -> String {
        let mut post_data = std::collections::HashMap::new();
        post_data.insert("status", to_status.to_string());
        post_data.insert("task_id_type", task_id.to_string());

        let url = format!("{}}/{}/{}", self.obj_id, self.archivist);
        // let resp 
        // let resp = reqwest::post().form(&post_data).send().await?;
        return after_update_task(resp).await.unwrap();
    }
}

async fn after_update_task(r: reqwest::Response) -> Option<String> {
    let status = r.status();
    let body = r.text().await.ok()?;
    if status == 200 {
        Some(body)
    }
    /* if r.status() == 400 { panic!(body); } */
    else {
        panic!("{}", body);
    }
}

/*
type ObjectID string
type DatetimeUTC string

func (d DatetimeUTC) GetDatetime() time.Time {
    // 2024-06-06T15:45:00.008Z
    t, err := time.Parse(time.RFC3339, string(d))
    if err != nil {
        panic(err)
    }
    return t
}


var (
    ErrorClientVersionOutdated = errors.New("client version outdated")
    ENABLE_GZIP                = true
)

func (t *Tracker) ClaimTask(with_delay bool) *Task {
    if with_delay {
        t._claim_wait_lock.Lock()
        time.Sleep(time.Duration(t.Project().Client.ClaimTaskDelay * float64(time.Second)))
        t._claim_wait_lock.Unlock()
    }
    resp, err := t.HTTP_client.Post(t.API_BASE+t.API_VERSION+"/project/"+t.project_id+"/"+t.client_version+"/"+t.archivist+"/claim_task", "", nil)
    if err != nil {
        panic(err)
    }
    return _after_claim_task(resp)
}

// 无任务返回 nil
func _after_claim_task(r *http.Response) *Task {
    if r.StatusCode == 404 {
        return nil // 无任务
    }
    if r.StatusCode == 200 {
        task := Task{}
        err := json.NewDecoder(r.Body).Decode(&task)
        if err != nil {
            panic(err)
        }

        var idInt int
        var idString string
        if err := json.Unmarshal(task.Id_raw, &idInt); err == nil {
            idString = fmt.Sprintf("%d", idInt)
            task.Id = idString
            task.Id_type = "int"
        } else if err := json.Unmarshal(task.Id_raw, &idString); err == nil {
            task.Id = idString
            task.Id_type = "str"
        } else {
            panic(err)
        }

        return &task
    }

    BodyBytes, _ := io.ReadAll(r.Body)
    panic(string(BodyBytes))
}

func (t *Tracker) UpdateTask(task_id string, id_type string, to_status Status) string {

    postData := url.Values{}
    postData.Set("status", string(to_status))
    postData.Set("task_id_type", id_type)

    if !to_status.Validate() {
        fmt.Println("invalid status, to_status:", to_status)
        panic("invalid status")
    }

    resp, err := t.HTTP_client.Post(t.API_BASE+t.API_VERSION+"/project/"+t.project_id+"/"+t.client_version+"/"+t.archivist+"/update_task/"+task_id, "application/x-www-form-urlencoded", strings.NewReader(postData.Encode()))
    if err != nil {
        panic(err)
    }
    return _after_update_task(resp)
}

func _after_update_task(r *http.Response) string {
    bodyBytes, err := io.ReadAll(r.Body)
    if err != nil {
        panic(err)
    }
    text := string(bodyBytes)

    if r.StatusCode == 200 {
        return text
    }
    if r.StatusCode == 400 {
        panic(text)
    }

    fmt.Println(r.StatusCode, r.Request.URL, text)
    panic(text)
}

func (t *Tracker) InsertMany(Items []Item) string {
    if len(Items) == 0 {
        return "len(Items) == 0, nothing to insert"
    }
    req_url := t.API_BASE + t.API_VERSION + "/project/" + t.project_id + "/" + t.client_version + "/" + t.archivist + "/insert_many/" + fmt.Sprintf("%d", len(Items))

    items_json_str, err := json.Marshal(Items)
    if err != nil {
        panic(err)
    }
    len_encodedData := len(items_json_str)

    gzBuf, err := t.GzCompress(items_json_str)
    if err != nil {
        panic(err)
    }

    req := &http.Request{}

    if ENABLE_GZIP && float64(gzBuf.Len())/float64(len_encodedData) < 0.95 { // good compression rate
        req, err = http.NewRequest("POST", req_url, gzBuf)
        if err != nil {
            panic(err)
        }
        req.Header.Set("Content-Encoding", "gzip")
    } else {
        req, err = http.NewRequest("POST", req_url, bytes.NewReader(items_json_str))
        if err != nil {
            panic(err)
        }
    }

    req.Header.Set("Content-Type", "application/json; charset=utf-8")
    req.Header.Set("Accept", "* / *")

    resp, err := t.HTTP_client.Do(req)
    if err != nil {
        panic(err)
    }
    return _after_insert_item(resp)
}

func (t *Tracker) GzCompress(data []byte) (*bytes.Buffer, error) {
    gzBuf := &bytes.Buffer{}

    gz := t.__gzPool.Get().(*gzip.Writer)
    defer t.__gzPool.Put(gz)
    defer gz.Reset(io.Discard)
    defer gz.Close()

    gz.Reset(gzBuf)
    if _, err := gz.Write(data); err != nil {
        return nil, err
    }
    if err := gz.Flush(); err != nil {
        return nil, err
    }
    gz.Close()

    return gzBuf, nil
}

func (t *Tracker) InsertItem(task Task, item_status string, status_type string, payload string) string {
    if status_type != "int" && status_type != "str" && status_type != "None" {
        panic("status must be int, str or None")
    }
    req_url := t.API_BASE + t.API_VERSION + "/project/" + t.project_id + "/" + t.client_version + "/" + t.archivist + "/insert_item/" + task.Id

    var err error
    item := Item{
        Item_id:          task.Id,
        Item_id_type:     task.Id_type,
        Item_status:      item_status,
        Item_status_type: status_type,
        Payload:          payload,
    }
    data, err := json.Marshal(item)
    if err != nil {
        panic(err)
    }
    len_data := len(data)

    gzBuf, err := t.GzCompress(data)
    if err != nil {
        panic(err)
    }

    // fmt.Printf("compressed %d -> %d \n", len_encodedData, gzBuf.Len())
    req := &http.Request{}

    if ENABLE_GZIP && float64(gzBuf.Len())/float64(len_data) < 0.95 { // good compression rate
        req, err = http.NewRequest("POST", req_url, gzBuf)
        if err != nil {
            panic(err)
        }
        req.Header.Set("Content-Encoding", "gzip")
    } else {
        req, err = http.NewRequest("POST", req_url, bytes.NewReader(data))
        if err != nil {
            panic(err)
        }
    }

    req.Header.Set("Content-Type", "application/json; charset=utf-8")
    req.Header.Set("Accept", "* / *")

    resp, err := t.HTTP_client.Do(req)
    if err != nil {
        panic(err)
    }
    return _after_insert_item(resp)
}

func _after_insert_item(r *http.Response) string {
    defer r.Body.Close()
    bodyBytes, err := io.ReadAll(r.Body)
    if err != nil {
        panic(err)
    }
    text := string(bodyBytes)

    if r.StatusCode == 200 {
        return text
    }

    fmt.Println(r.StatusCode, r.Request.URL, text)
    panic(text)
}

*/
