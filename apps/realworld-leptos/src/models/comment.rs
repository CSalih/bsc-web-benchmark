use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Comment {
    pub id: i32,
    pub article: String,
    pub username: String,
    pub body: String,
    pub created_at: String,
    pub user_image: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommentsResponse {
    pub comments: Vec<Comment>,
}

impl Comment {
    pub async fn load_comments(slug: String) -> CommentsResponse {
        let url = format!("http://localhost:8080/api/articles/{slug}/comments");

        reqwest::Client::new()
            .get(url)
            .send()
            .await
            .unwrap()
            .json::<CommentsResponse>()
            .await
            .unwrap()
    }
}
