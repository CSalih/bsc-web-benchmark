use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Hash, PartialOrd, Eq)]
pub struct Pagination {
    tag: Option<String>,
    my_feed: Option<bool>,
    page: Option<u32>,
    amount: Option<u32>,
}

impl Pagination {
    #[inline]
    pub fn get_tag(&self) -> &str {
        self.tag.as_deref().unwrap_or_default()
    }
    #[inline]
    pub fn get_my_feed(&self) -> bool {
        self.my_feed.unwrap_or_default()
    }
    #[inline]
    pub fn get_page(&self) -> u32 {
        self.page.unwrap_or_default()
    }
    #[inline]
    pub fn get_amount(&self) -> u32 {
        self.amount.unwrap_or(10)
    }

    #[inline]
    pub fn set_my_feed(mut self, feed: bool) -> Self {
        self.my_feed = Some(feed);
        self
    }

    #[inline]
    pub fn reset_page(mut self) -> Self {
        self.page = Some(1);
        self
    }

    #[inline]
    pub fn set_page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            tag: Some(String::new()),
            my_feed: Some(false),
            page: Some(1),
            amount: Some(10),
        }
    }
}

impl Display for Pagination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!(
            "/?tag={}&my_feed={}&page={}&amount={}",
            self.get_tag(),
            self.get_my_feed(),
            self.get_page(),
            self.get_amount(),
        );
        write!(f, "{}", str)
    }
}
