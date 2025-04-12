use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "type")]
    pub event_type: String,
    pub actor: Actor,
    pub payload: Value,
    pub repo: Repo,
}

#[derive(Deserialize, Debug)]
pub struct Repo {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Actor {
    pub display_login: String,
}
