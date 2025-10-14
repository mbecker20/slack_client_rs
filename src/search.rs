//! These methods are only available after calling
//! `.with_api_token("token")` on the client.
//!
//! The token must have `search:read` scope.

use std::time::Duration;

use anyhow::{anyhow, Context as _};
use reqwest::{Response, StatusCode};
use serde::{Deserialize, Serialize};

use crate::Client;

impl Client {
    /// When posting to threads, need to find the thread ts.
    /// Right after posting message, it can take a little bit to show up,
    /// this retries the query until a ts after the post ts is found,
    /// waiting 3 seconds in between tries, with max of 10 tries.
    pub async fn find_thread_ts(
        &self,
        query: SearchQuery<'_>,
        post_ts_seconds: f64,
    ) -> anyhow::Result<String> {
        let query = query.count(1);

        for _try in 0..10 {
            let Some((thread_ts_f64, thread_ts)) =
                self.search(query).await?.messages.and_then(|mut messages| {
                    messages
                        .matches
                        .pop()
                        .and_then(|message| Some((message.ts.parse::<f64>().ok()?, message.ts)))
                })
            else {
                tokio::time::sleep(Duration::from_secs(3)).await;
                continue;
            };
            if thread_ts_f64 >= post_ts_seconds {
                return Ok(thread_ts);
            } else {
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }

        Err(anyhow!(
            "Did not find message with ts greater than {post_ts_seconds}"
        ))
    }

    /// https://docs.slack.dev/reference/methods/search.messages/
    pub async fn search(&self, query: SearchQuery<'_>) -> anyhow::Result<SearchResponse> {
        let res = self
            .get("https://slack.com/api/search.messages", &query)
            .await?;
        res.json().await.context("Failed to parse response body")
    }

    async fn get<'a, T: Serialize>(&self, url: &str, query: &T) -> anyhow::Result<Response> {
        let res =
            self.http_client()
                .get(url)
                .header("Content-Type", "application/json")
                .header(
                    "Authorization",
                    format!("Bearer {}", self.api_token.as_ref().context(
                    "Cannot call query apis without calling .with_api_token() on the Client.",
                )?),
                )
                .query(query)
                .send()
                .await?;
        let status = res.status();
        if status == StatusCode::OK {
            Ok(res)
        } else {
            let text = res.text().await.context(format!("status: {status}"))?;
            Err(anyhow!("status: {status} | text: {text}"))
        }
    }
}

/// https://docs.slack.dev/reference/methods/search.messages/
#[derive(Debug, Clone, Copy, Serialize)]
pub struct SearchQuery<'a> {
    /// Example: `in:channel_name`
    pub query: &'a str,
    /// From 1-100. Default is 20.
    pub count: Option<u32>,
    /// From 1-100. Default is 1.
    pub page: Option<u32>,
    /// Pass a value of true to enable query highlight markers.
    pub highlight: Option<bool>,
    /// Choose the sorting direction
    pub sort_dir: Option<SortDir>,
}

impl<'a> SearchQuery<'a> {
    pub fn new(query: &'a str) -> Self {
        Self {
            query,
            count: None,
            page: None,
            highlight: None,
            sort_dir: None,
        }
    }
}

impl SearchQuery<'_> {
    /// From 1-100. Default is 20.
    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }
    /// From 1-100. Default is 1.
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }
    /// Pass a value of true to enable query highlight markers.
    pub fn highlight(mut self, highlight: bool) -> Self {
        self.highlight = Some(highlight);
        self
    }
    /// Choose the sorting direction
    pub fn sort_dir(mut self, sort_dir: SortDir) -> Self {
        self.sort_dir = Some(sort_dir);
        self
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SortDir {
    Desc,
    Asc,
}

/// https://docs.slack.dev/reference/methods/search.messages/
#[derive(Debug, Clone, Deserialize)]
pub struct SearchResponse {
    pub messages: Option<SearchResponseMessages>,
    pub ok: bool,
    pub query: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResponseMessages {
    pub matches: Vec<SearchResponseMessagesMatch>,
    pub pagination: SearchResponseMessagesPagination,
    pub paging: SearchResponseMessagesPaging,
    pub total: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResponseMessagesMatch {
    pub channel: SearchResponseMessagesMatchChannel,
    pub iid: String,
    pub permalink: String,
    pub team: String,
    pub text: String,
    pub ts: String,
    #[serde(rename = "type")]
    pub match_type: String,
    pub user: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResponseMessagesMatchChannel {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResponseMessagesPagination {
    pub first: u32,
    pub last: u32,
    pub page: u32,
    pub page_count: u32,
    pub per_page: u32,
    pub total_count: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResponseMessagesPaging {
    pub count: u32,
    pub page: u32,
    pub pages: u32,
    pub total: u32,
}
