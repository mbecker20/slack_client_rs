use anyhow::{anyhow, Context};
use reqwest::{Response, StatusCode};

pub mod types;

use serde::Serialize;
use types::*;

#[derive(Debug, Clone)]
pub struct Client {
    url: String,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(url: impl Into<String>) -> Client {
        Client {
            url: url.into(),
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn send_message<'a>(
        &self,
        text: &'a str,
        blocks: impl Into<Option<&'a [Block<'a>]>>,
    ) -> anyhow::Result<Response> {
        self.post(&SlackMessageBody {
            text,
            blocks: blocks.into(),
        })
        .await
    }

    pub async fn send_message_with_header(
        &self,
        header: &str,
        info: impl Into<Option<&str>>,
    ) -> anyhow::Result<Response> {
        let blocks = match info.into() {
            Some(info) => Some(vec![Block::header(header), Block::section(info)]),
            None => Some(vec![Block::header(header)]),
        };
        self.send_message(header, blocks.as_deref()).await
    }

    pub async fn send_mrkdwn_message(&self, text: &str) -> anyhow::Result<Response> {
        self.send_message(&text, vec![Block::section(text)].as_slice())
            .await
    }

    pub async fn send_owned_message<'a>(
        &self,
        text: &'a str,
        blocks: impl AsRef<[OwnedBlock]>,
    ) -> anyhow::Result<()> {
        self.send_owned_message_manually_chunked(text, blocks.as_ref(), 50)
            .await
    }

    pub async fn send_owned_message_manually_chunked<'a>(
        &self,
        text: &'a str,
        blocks: impl AsRef<[OwnedBlock]>,
        chunks: usize,
    ) -> anyhow::Result<()> {
        for blocks in blocks.as_ref().chunks(chunks) {
            self.send_owned_message_single(text, blocks).await?;
        }
        Ok(())
    }

    pub async fn send_owned_message_single<'a>(
        &self,
        text: &'a str,
        blocks: impl Into<Option<&[OwnedBlock]>>,
    ) -> anyhow::Result<Response> {
        self.post(&OwnedSlackMessageBody {
            text,
            blocks: blocks.into(),
        })
        .await
    }

    async fn post<'a, T: Serialize>(&self, json: &T) -> anyhow::Result<Response> {
        let res = self
            .http_client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(json)
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
