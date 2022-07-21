use reqwest::Response;

use super::types::{Block, SlackMessageBody};

#[derive(Debug, Clone)]
pub struct Client {
    url: String,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(url: &str) -> Client {
        Client {
            url: url.to_string(),
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn send_message(
        &self,
        text: impl Into<String>,
        blocks: impl Into<Option<Vec<Block>>>,
    ) -> Result<Response, reqwest::Error> {
        self.http_client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&SlackMessageBody {
                text: text.into(),
                blocks: blocks.into(),
            })
            .send()
            .await
    }

    pub async fn send_formatted_message(
        &self,
        header: impl Into<String>,
        info: impl Into<String>,
    ) -> Result<Response, reqwest::Error> {
        let header: String = header.into();
        self.http_client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&SlackMessageBody {
                text: header.clone(),
                blocks: Some(vec![Block::header(header), Block::section(info)]),
            })
            .send()
            .await
    }
}
