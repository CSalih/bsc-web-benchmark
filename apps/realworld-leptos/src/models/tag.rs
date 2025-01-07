use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug)]
pub struct Tag;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TagsResponse {
    pub tags: Vec<String>,
}

impl Tag {
    pub async fn load_tags() -> TagsResponse {
        let url = "http://localhost:8080/api/tags";

        reqwest::Client::new()
            .get(url)
            .send()
            .await
            .unwrap()
            .json::<TagsResponse>()
            .await
            .unwrap()
    }
}
