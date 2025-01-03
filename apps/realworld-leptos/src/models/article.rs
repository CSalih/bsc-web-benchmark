use super::{Pagination, UserPreview};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub slug: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    pub description: String,
    pub created_at: String,
    pub favorites_count: i64,
    pub tag_list: Vec<String>,
    pub author: UserPreview,
    pub favorited: bool,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ArticlesResponse {
    pub articles: Vec<Article>,
    pub articles_count: i64,
}

impl Article {
    pub async fn load_articles(pagination: Pagination) -> ArticlesResponse {
        let url = "http://localhost:8080/api/articles";
        let mut query = vec![
            ("limit", pagination.get_amount().to_string()),
            ("offset", ((pagination.get_page() - 1) * pagination.get_amount()).to_string()),
        ];
        if !pagination.get_tag().is_empty() {
            query.push(("tag", pagination.get_tag().into()));
        }
        if pagination.get_my_feed() {
            query.push(("feed", "true".into()));
        }

        reqwest::Client::new()
            .get(url)
            .query(&query)
            .send()
            .await
            .unwrap()
            .json::<ArticlesResponse>()
            .await
            .unwrap()
    }
}
