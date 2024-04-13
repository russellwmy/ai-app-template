#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TextElementGroupKind {
    Heading,
    BulletList,
    ReferenceList,
    None,
    Paragraph,
    Table,
    PageNumber,
}
