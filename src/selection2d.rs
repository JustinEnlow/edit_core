use crate::Position;

/// 2 dimensional representation of a single selection(between anchor and head) within document text
#[derive(Default, PartialEq, Debug, Clone)]
pub struct Selection2d{
    anchor: Position,
    head: Position, //TODO: should this be cursor? because we are using cursor in selection_to_selection2d...
}
impl Selection2d{
    pub fn new(anchor: Position, head: Position) -> Self{
        Self{
            anchor,
            head
        }
    }
    pub fn head(&self) -> &Position{
        &self.head
    }
    pub fn anchor(&self) -> &Position{
        &self.anchor
    }
}
