use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Serialize)]
pub struct BlockText<'a> {
    #[serde(rename = "type")]
    pub txt_type: TextType,
    pub text: &'a str,
}

#[derive(Debug, Serialize)]
pub struct Block<'a> {
    #[serde(rename = "type")]
    pub msg_type: MsgType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<BlockText<'a>>,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TextType {
    PlainText,
    Mrkdwn,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum MsgType {
    Header,
    Section,
    Divider,
}

impl Block<'_> {
    pub fn new<'a>(
        msg_type: MsgType,
        txt_type: TextType,
        text: impl Into<Option<&'a str>>,
    ) -> Block<'a> {
        Block {
            msg_type,
            text: text.into().map(|text| BlockText { txt_type, text }),
        }
    }

    pub fn header<'a>(text: &'a str) -> Block<'a> {
        Block::new(MsgType::Header, TextType::PlainText, text)
    }

    pub fn section<'a>(text: &'a str) -> Block<'a> {
        Block::new(MsgType::Section, TextType::Mrkdwn, text)
    }

    pub fn divider<'a>() -> Block<'a> {
        Block {
            msg_type: MsgType::Divider,
            text: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SlackMessageBody<'a> {
    pub text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<&'a [Block<'a>]>,
}
