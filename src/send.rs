use anyhow::{anyhow, Context as _};
use reqwest::{Response, StatusCode};
use serde::Serialize;

use crate::{
    types::{Block, OwnedBlock, OwnedSlackMessageBody, SlackMessageBody},
    Client,
};

impl Client {
    pub async fn send_message<'a>(
        &self,
        text: &'a str,
        thread_ts: impl Into<Option<&'a str>>,
        blocks: impl Into<Option<&'a [Block<'a>]>>,
    ) -> anyhow::Result<Response> {
        self.post(&SlackMessageBody {
            text,
            thread_ts: thread_ts.into(),
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
        self.send_message(header, None, blocks.as_deref()).await
    }

    pub async fn send_mrkdwn_message<'a>(
        &self,
        thread_ts: impl Into<Option<&'a str>>,
        text: &str,
    ) -> anyhow::Result<Response> {
        self.send_message(
            &text,
            thread_ts.into(),
            vec![Block::section(text)].as_slice(),
        )
        .await
    }

    pub async fn send_owned_message<'a>(
        &self,
        text: &'a str,
        thread_ts: impl Into<Option<&'a str>>,
        blocks: impl AsRef<[OwnedBlock]>,
    ) -> anyhow::Result<()> {
        self.send_owned_message_manually_chunked(text, thread_ts.into(), blocks.as_ref(), 50)
            .await
    }

    pub async fn send_owned_message_manually_chunked<'a>(
        &self,
        text: &'a str,
        thread_ts: impl Into<Option<&'a str>>,
        blocks: impl AsRef<[OwnedBlock]>,
        chunks: usize,
    ) -> anyhow::Result<()> {
        let thread_ts = thread_ts.into();
        for blocks in blocks.as_ref().chunks(chunks) {
            self.send_owned_message_single(text, thread_ts, blocks)
                .await?;
        }
        Ok(())
    }

    pub async fn send_owned_message_single<'a>(
        &self,
        text: &'a str,
        thread_ts: impl Into<Option<&'a str>>,
        blocks: impl Into<Option<&[OwnedBlock]>>,
    ) -> anyhow::Result<Response> {
        self.post(&OwnedSlackMessageBody {
            text,
            thread_ts: thread_ts.into(),
            blocks: blocks.into(),
        })
        .await
    }

    async fn post<T: Serialize>(&self, json: &T) -> anyhow::Result<Response> {
        let res = self
            .http_client()
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
