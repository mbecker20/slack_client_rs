use reqwest::{Response, StatusCode};

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
    ) -> Result<Response, String> {
        let res = self.http_client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&SlackMessageBody {
                text: text.into(),
                blocks: blocks.into(),
            })
            .send()
            .await;
        match res {
            Ok(res) => {
                let status = res.status();
                if status == StatusCode::OK {
                    Ok(res)
                } else {
                    match res.text().await {
                        Ok(res) => Err(format!("{status}: {res}")),
                        Err(e) => Err(format!("{status}: {e:#?}"))
                    }
                }
            }
            Err(e) => Err(format!("{e:#?}")),
        }
    }

    pub async fn send_message_with_header(
        &self,
        header: impl Into<String>,
        info: impl Into<Option<String>>,
    ) -> Result<Response, String> {
        let header: String = header.into();
        let info: Option<String> = info.into();
        let blocks = match &info {
            Some(info) => Some(vec![Block::header(header.clone()), Block::section(info)]),
            None => Some(vec![Block::header(header.clone())]),
        };
        let res = self.http_client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&SlackMessageBody {
                text: header,
                blocks,
            })
            .send()
            .await;
        match res {
            Ok(res) => {
                let status = res.status();
                if status == StatusCode::OK {
                    Ok(res)
                } else {
                    match res.text().await {
                        Ok(res) => Err(format!("{status}: {res}")),
                        Err(e) => Err(format!("{status}: {e:#?}"))
                    }
                }
            }
            Err(e) => Err(format!("{e:#?}")),
        }
    }

    pub async fn send_mrkdwn_message(&self, text: impl Into<String>) -> Result<Response, String> {
        let text = text.into();
        self.send_message(&text, vec![Block::section(&text)]).await
    }
}
