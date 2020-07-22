pub enum ListItemType {
    Ordered,
    Unordered,
}


pub enum BlockType {
    Heading(u8)
    ListItem(ListItemType),
    Paragraph,
    Quote,
}

pub enum InlineType {
    Bold,
    Italic,
    Link(String),
}

type Inline = (InlineType, Inline)
type Block = (BlockType, Block)
