use crate::document::{List, Node};

pub(crate) fn group_list_items(nodes: Vec<Node>) -> Vec<Node> {
    let mut result = vec![];
    let mut buf = vec![];
    for node in nodes {
        match node {
            Node::ListItem(_) => buf.push(node),
            _ => {
                if !buf.is_empty() {
                    result.push(Node::List(
                        List::builder()
                            .children(buf.clone())
                            .start(None)
                            .position(None)
                            .spread(true)
                            .ordered(false)
                            .build(),
                    ));
                    result.push(Node::LineBreak);
                    buf = vec![];
                }
                result.push(node);
            }
        }
    }
    if !buf.is_empty() {
        result.push(Node::List(
            List::builder()
                .children(buf.clone())
                .start(None)
                .position(None)
                .spread(true)
                .ordered(false)
                .build(),
        ));
        result.push(Node::LineBreak);
    }
    result
}
