#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct Comment {
    pub id: i32,
    pub article: String,
    pub username: String,
    pub body: String,
    pub created_at: String,
    pub user_image: Option<String>,
}
