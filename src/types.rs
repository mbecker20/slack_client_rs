use serde_derive::{Serialize, Deserialize};
use strum_macros::{Display, EnumString};

#[derive(Serialize)]
pub struct BlockText {
    #[serde(rename = "type")]
    pub txt_type: String,
    pub text: String,
}

#[derive(Serialize)]
pub struct Block {
    #[serde(rename = "type")]
    pub msg_type: String,
    pub text: BlockText,
}

#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TextType {
    PlainText,
    Mrkdwn,
}

#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MsgType {
    Header,
    Section,
}

impl Block {
    pub fn new(msg_type: MsgType, txt_type: TextType, text: &str) -> Block {
        Block {
            msg_type: msg_type.to_string(),
            text: BlockText {
                txt_type: txt_type.to_string(),
                text: text.to_string(),
            },
        }
    }
}

#[derive(Serialize)]
pub struct SlackMessageBody {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Block>>,
}
