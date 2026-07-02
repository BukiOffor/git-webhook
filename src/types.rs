use serde::Deserialize;

#[derive(Deserialize)]
pub struct PushEvent {
    #[serde(rename = "ref")]
    pub ref_: String,

    pub repository: Repository,
    pub head_commit: Commit,
}

#[derive(Deserialize)]
pub struct Repository {
    pub full_name: String,
}

#[derive(Deserialize)]
pub struct Commit {
    pub id: String,
    pub message: String,
}
