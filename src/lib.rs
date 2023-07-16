use anyhow::{anyhow, Context};
use reqwest::{Response, StatusCode};

pub mod types;

use types::*;

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
    ) -> anyhow::Result<Response> {
        let res = self
            .http_client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&SlackMessageBody {
                text: text.into(),
                blocks: blocks.into(),
            })
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

    pub async fn send_message_with_header(
        &self,
        header: impl Into<String>,
        info: impl Into<Option<String>>,
    ) -> anyhow::Result<Response> {
        let header: String = header.into();
        let info: Option<String> = info.into();
        let blocks = match &info {
            Some(info) => Some(vec![Block::header(header.clone()), Block::section(info)]),
            None => Some(vec![Block::header(header.clone())]),
        };
        self.send_message(header, blocks).await
    }

    pub async fn send_mrkdwn_message(&self, text: impl Into<String>) -> anyhow::Result<Response> {
        let text = text.into();
        self.send_message(&text, vec![Block::section(&text)]).await
    }
}
