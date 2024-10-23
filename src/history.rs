use crate::selection::Selection;

#[derive(Clone, Debug, PartialEq)]
pub enum Operation{
    Insert,
    Delete,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Change{
    operation: Operation,
    text: String,
    old_selection: Selection,
    new_selection: Selection,
}
impl Change{
    pub fn new(operation: Operation, text: String, old_selection: Selection, new_selection: Selection) -> Self{
        Self{
            operation,
            text,
            old_selection,
            new_selection
        }
    }
    pub fn operation(&self) -> Operation{
        self.operation.clone()
    }
    pub fn text(&self) -> String{
        self.text.clone()
    }
    pub fn new_selection(&self) -> Selection{
        self.new_selection.clone()
    }
    pub fn old_selection(&self) -> Selection{
        self.old_selection.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChangeSet{
    changes: Vec<Change>,
}
impl ChangeSet{
    pub fn new(changes: Vec<Change>) -> Self{
        Self{changes}
    }
    pub fn changes(&self) -> Vec<Change>{
        self.changes.clone()
    }
    /// ```
    /// # use edit_core::selection::Selection;
    /// # use edit_core::history::{Operation, Change, ChangeSet};
    /// 
    /// 
    /// // "|>"
    /// // "idk\n|>"
    /// let original_changes = vec![Change::new(Operation::Insert, "idk\n".to_string(), Selection::new(0, 0), Selection::with_stored_line_position(4, 4, 0))];
    /// //let inverse_changes = vec![Change::new(Operation::Delete, "idk\n".to_string(), Selection::new(4, 4), Selection::new(0, 0))];
    /// // maybe inverse_changes should be     Operation::Delete, "idk\n".to_string(), Selection::new(0, 4), Selection::new(0, 0)?
    /// let inverse_changes = vec![Change::new(Operation::Delete, "idk\n".to_string(), Selection::new(0, 4), Selection::new(0, 0))];
    /// assert_eq!(inverse_changes, ChangeSet::new(original_changes).invert());
    /// 
    /// // "idk\n|>"
    /// // "|>"
    /// let original_changes = vec![Change::new(Operation::Delete, "idk\n".to_string(), Selection::new(4, 4), Selection::with_stored_line_position(0, 0, 0))];
    /// let inverse_changes = vec![Change::new(Operation::Insert, "idk\n".to_string(), Selection::new(0, 0), Selection::new(4, 4))];
    /// assert_eq!(inverse_changes, ChangeSet::new(original_changes).invert());
    /// ```
    pub fn invert(&self) -> Vec<Change>{
        let mut new_changes = Vec::new();
            
        for change in self.changes(){
            match change.operation(){
                Operation::Insert => {
                    // Create a Change that represents the deletion of the inserted text
                    let undo_change = Change::new(
                        Operation::Delete,
                        change.text(),
                        //Selection::new(change.new_selection.anchor(), change.new_selection.head()),
                        Selection::new(change.old_selection.anchor(), change.new_selection.anchor()),
                        Selection::new(change.old_selection.anchor(), change.old_selection.head())
                    );
                    new_changes.push(undo_change);
                }
                Operation::Delete => {
                    // Create a Change that represents the insertion of the deleted text
                    let undo_change = Change::new(
                        Operation::Insert,
                        change.text(),
                        Selection::new(change.new_selection.anchor(), change.new_selection.head()),
                        Selection::new(change.old_selection.anchor(), change.old_selection.head())
                    );
                    new_changes.push(undo_change);
                }
            }
        }

        new_changes
    }
}
impl Iterator for ChangeSet{
    type Item = Change;

    fn next(&mut self) -> Option<Self::Item>{
        self.changes.pop()
    }
}
