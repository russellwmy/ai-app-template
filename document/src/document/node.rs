use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use super::{
    BlockQuote, Code, Heading, Image, InlineCode, InlineMath, List, ListItem, Math, Paragraph,
    Position, Table, TableCell, TableRow, Text, TextGroup,
};
use crate::Result;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Node {
    BlockQuote(BlockQuote),
    Code(Code),
    Heading(Heading),
    Image(Image),
    InlineCode(InlineCode),
    InlineMath(InlineMath),
    LineBreak,
    List(List),
    ListItem(ListItem),
    Math(Math),
    Paragraph(Paragraph),
    Table(Table),
    TableRow(TableRow),
    TableCell(TableCell),
    Text(Text),
    TextGroup(TextGroup),
}

impl Node {
    pub fn position(&self) -> Option<Position> {
        match self {
            Node::BlockQuote(o) => o.position().to_owned(),
            Node::Code(o) => o.position().to_owned(),
            Node::Heading(o) => o.position().to_owned(),
            Node::Image(o) => o.position().to_owned(),
            Node::InlineCode(o) => o.position().to_owned(),
            Node::InlineMath(o) => o.position().to_owned(),
            Node::LineBreak => None,
            Node::List(o) => o.position().to_owned(),
            Node::ListItem(o) => o.position().to_owned(),
            Node::Math(o) => o.position().to_owned(),
            Node::Paragraph(o) => o.position().to_owned(),
            Node::Table(o) => o.position().to_owned(),
            Node::TableRow(o) => o.position().to_owned(),
            Node::TableCell(o) => o.position().to_owned(),
            Node::Text(o) => o.position().to_owned(),
            Node::TextGroup(o) => o.position().to_owned(),
        }
    }

    pub fn is_positioal(&self) -> bool {
        match self {
            Node::LineBreak => false,
            _ => true,
        }
    }
}

impl Node {
    pub fn as_block_quote(&self) -> Result<BlockQuote> {
        match self {
            Node::BlockQuote(node) => Ok(node.to_owned()),
            _ => Err(anyhow!("Node is not a block qoute.")),
        }
    }

    pub fn as_code(&self) -> Result<Code> {
        match self {
            Node::Code(node) => Ok(node.to_owned()),
            _ => Err(anyhow!("Node is not a code.")),
        }
    }

    pub fn as_heading(&self) -> Result<Heading> {
        match self {
            Node::Heading(node) => Ok(node.to_owned()),
            _ => Err(anyhow!("Node is not a heading.")),
        }
    }

    pub fn as_image(&self) -> Result<Image> {
        match self {
            Node::Image(node) => Ok(node.to_owned()),
            _ => Err(anyhow!("Node is not an image.")),
        }
    }

    pub fn as_inline_code(&self) -> Result<InlineCode> {
        match self {
            Node::InlineCode(node) => Ok(node.to_owned()),
            _ => Err(anyhow!("Node is not an inline code.")),
        }
    }

    pub fn as_inline_math(&self) -> Result<InlineMath> {
        match self {
            Node::InlineMath(node) => Ok(node.to_owned()),
            _ => Err(anyhow!("Node is not an inline math.")),
        }
    }

    pub fn as_list(&self) -> Result<List> {
        match self {
            Node::List(node) => Ok(node.to_owned()),
            _ => Err(anyhow!("Node is not a list.")),
        }
    }

    pub fn as_list_item(&self) -> Result<ListItem> {
        match self {
            Node::ListItem(node) => Ok(node.to_owned()),
            _ => Err(anyhow!("Node is not a list itme.")),
        }
    }

    pub fn as_math(&self) -> Result<Math> {
        match self {
            Node::Math(node) => Ok(node.to_owned()),
            _ => Err(anyhow!("Node is not a math.")),
        }
    }

    pub fn as_paragraph(&self) -> Result<Paragraph> {
        match self {
            Node::Paragraph(node) => Ok(node.to_owned()),
            _ => Err(anyhow!("Node is not a paragraph.")),
        }
    }

    pub fn as_table(&self) -> Result<Table> {
        match self {
            Node::Table(node) => Ok(node.to_owned()),
            _ => Err(anyhow!("Node is not a table.")),
        }
    }

    pub fn as_table_row(&self) -> Result<TableRow> {
        match self {
            Node::TableRow(node) => Ok(node.to_owned()),
            _ => Err(anyhow!("Node is not a table row.")),
        }
    }

    pub fn as_table_cell(&self) -> Result<TableCell> {
        match self {
            Node::TableCell(node) => Ok(node.to_owned()),
            _ => Err(anyhow!("Node is not a table cell.")),
        }
    }

    pub fn as_text(&self) -> Result<Text> {
        match self {
            Node::Text(node) => Ok(node.to_owned()),
            _ => Err(anyhow!("Node is not a text.")),
        }
    }

    pub fn as_text_group(&self) -> Result<TextGroup> {
        match self {
            Node::TextGroup(node) => Ok(node.to_owned()),
            _ => Err(anyhow!("Node is not a text group.")),
        }
    }
}
