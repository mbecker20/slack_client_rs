use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Serialize)]
pub struct BlockText {
    #[serde(rename = "type")]
    pub txt_type: TextType,
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct Block {
    #[serde(rename = "type")]
    pub msg_type: MsgType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<BlockText>,
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

impl Block {
    pub fn new(msg_type: MsgType, txt_type: TextType, text: impl Into<String>) -> Block {
        Block {
            msg_type,
            text: BlockText {
                txt_type,
                text: text.into(),
            }
            .into(),
        }
    }

    pub fn header(text: impl Into<String>) -> Block {
        Block::new(MsgType::Header, TextType::PlainText, text)
    }

    pub fn section(text: impl Into<String>) -> Block {
        Block::new(MsgType::Section, TextType::Mrkdwn, text)
    }

    pub fn divider() -> Block {
        Block {
            msg_type: MsgType::Divider,
            text: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SlackMessageBody {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Block>>,
}
