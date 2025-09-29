use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

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

#[derive(Debug, Serialize)]
pub struct SlackMessageBody<'a> {
    pub text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<&'a [Block<'a>]>,
}

#[derive(Debug, Serialize)]
pub struct Block<'a> {
    #[serde(rename = "type")]
    pub msg_type: MsgType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<BlockText<'a>>,
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
pub struct BlockText<'a> {
    #[serde(rename = "type")]
    pub txt_type: TextType,
    pub text: &'a str,
}

#[derive(Debug, Serialize)]
pub struct OwnedSlackMessageBody<'a> {
    pub text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<&'a [OwnedBlock]>,
}

#[derive(Debug, Clone, Default)]
pub struct OwnedBlocks(Vec<OwnedBlock>);

impl OwnedBlocks {
    pub fn push(&mut self, block: OwnedBlock) {
        self.0.push(block);
    }

    pub fn push_new(
        &mut self,
        msg_type: MsgType,
        txt_type: TextType,
        text: impl Into<Option<String>>,
    ) {
        self.0.push(OwnedBlock::new(msg_type, txt_type, text));
    }

    pub fn push_header(&mut self, text: impl Into<String>) {
        self.0.push(OwnedBlock::header(text));
    }

    pub fn push_section(&mut self, text: impl Into<String>) {
        self.0.push(OwnedBlock::section(text));
    }

    pub fn push_divider(&mut self) {
        self.0.push(OwnedBlock::divider());
    }

    pub fn chunks(&self, chunk_size: usize) -> impl Iterator<Item = &[OwnedBlock]> {
        self.0.chunks(chunk_size)
    }

    pub fn inner_mut(&mut self) -> &mut Vec<OwnedBlock> {
        &mut self.0
    }

    pub fn into_inner(self) -> Vec<OwnedBlock> {
        self.0
    }
}

impl AsRef<[OwnedBlock]> for OwnedBlocks {
    fn as_ref(&self) -> &[OwnedBlock] {
        &self.0
    }
}

impl From<Vec<OwnedBlock>> for OwnedBlocks {
    fn from(value: Vec<OwnedBlock>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OwnedBlock {
    pub msg_type: MsgType,
    pub text: Option<OwnedBlockText>,
}

impl OwnedBlock {
    pub fn new(
        msg_type: MsgType,
        txt_type: TextType,
        text: impl Into<Option<String>>,
    ) -> OwnedBlock {
        OwnedBlock {
            msg_type,
            text: text.into().map(|text| OwnedBlockText { txt_type, text }),
        }
    }

    pub fn header(text: impl Into<String>) -> OwnedBlock {
        OwnedBlock::new(MsgType::Header, TextType::PlainText, text.into())
    }

    pub fn section<'a>(text: impl Into<String>) -> OwnedBlock {
        OwnedBlock::new(MsgType::Section, TextType::Mrkdwn, text.into())
    }

    pub fn divider<'a>() -> OwnedBlock {
        OwnedBlock {
            msg_type: MsgType::Divider,
            text: None,
        }
    }

    pub fn as_block(&self) -> Block<'_> {
        Block {
            msg_type: self.msg_type,
            text: self.text.as_ref().map(OwnedBlockText::as_block_text),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct OwnedBlockText {
    pub txt_type: TextType,
    pub text: String,
}

impl OwnedBlockText {
    pub fn as_block_text(&self) -> BlockText<'_> {
        BlockText {
            txt_type: self.txt_type,
            text: self.text.as_str(),
        }
    }
}
