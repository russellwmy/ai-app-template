#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TextKind {
    Heading,
    Paragraph,
    List,
    ReferenceList,
    Table,
    Text,
    None,
}
