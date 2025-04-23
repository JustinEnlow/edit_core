use crate::{
    document::{Document, DocumentError},
    selection::CursorSemantics,
    history::Operation
};
use std::cmp::Ordering;

/// Reverts the last set of changes made to the document.
pub fn document_impl(document: &mut Document, semantics: CursorSemantics) -> Result<(), DocumentError>{    //should this be a HistoryError instead?...
    // Check if there is something to undo
    if let Some(change_set) = document.undo_stack.pop(){
        let changes = change_set.changes();
        
        document.selections = change_set.clone().selections_after_changes();    //set selections to selections_after_changes to account for any selection movements that may have occurred since edit
        assert!(document.selections.count() == changes.len());

        for (i, change) in changes.iter().enumerate().take(document.selections.count()){
            let selection = document.selections.nth_mut(i);
            match change.operation(){
                Operation::Insert{inserted_text} => {
                    selection.shift_and_extend(inserted_text.len(), &document.text, semantics);
                    let _ = Document::apply_delete(&mut document.text, selection, semantics);
                    document.selections.shift_subsequent_selections_backward(i, inserted_text.len());
                }
                Operation::Delete => {
                    if let Operation::Insert{inserted_text} = change.inverse(){
                        let _ = Document::apply_insert(&mut document.text, &inserted_text, selection, semantics);   //apply inverse operation
                        document.selections.shift_subsequent_selections_forward(i, inserted_text.len());
                    }
                }
                Operation::Replace{replacement_text} => {
                    let inserted_text = replacement_text;
                    if let Operation::Replace{replacement_text} = change.inverse(){
                        selection.shift_and_extend(inserted_text.len(), &document.text, semantics);
                        let _ = Document::apply_replace(&mut document.text, &replacement_text, selection, semantics);
                        match inserted_text.len().cmp(&replacement_text.len()){    //old selected text vs new text
                            Ordering::Greater => {document.selections.shift_subsequent_selections_backward(i, inserted_text.len().saturating_sub(replacement_text.len()));}
                            Ordering::Less => {document.selections.shift_subsequent_selections_forward(i, replacement_text.len().saturating_sub(inserted_text.len()));}
                            Ordering::Equal => {}   // no change to subsequent selections
                        }
                    }
                }
                Operation::NoOp => {}
            }
        }
        // selections should be the same as they were before changes were made, because we are restoring that previous state
        document.selections = change_set.selections_before_changes();

        // Push inverted changes onto redo stack
        document.redo_stack.push(change_set);

        Ok(())
    }else{Err(DocumentError::NoChangesToUndo)}
}

#[cfg(test)]
mod tests{
    use crate::utilities::undo;
    use crate::{
        document::Document, 
        selections::Selections,
        selection::{CursorSemantics, Selection, Direction}, 
        range::Range, 
        history::{Change, ChangeSet, Operation}, 
    };
    use ropey::Rope;

    //TODO: impl needed functions on Document + test
    fn test(text: &str, selections: Vec<Selection>, primary: usize, undo_stack: Vec<ChangeSet>, last_saved_text: &str, expected_text: &str, expected_selections: Vec<Selection>, expected_primary: usize, semantics: CursorSemantics){
        let text = Rope::from(text);
        let last_saved_text = Rope::from(last_saved_text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text, semantics))
            .with_undo_stack(undo_stack)
            .with_last_saved_text(last_saved_text);
        let _ = undo::document_impl(&mut doc, semantics);
        let expected_text = Rope::from(expected_text);
        assert_eq!(expected_text, doc.text);
        let expected_selections = Selections::new(expected_selections, expected_primary, &text, semantics);
        assert_eq!(expected_selections, doc.selections);
        assert!(!doc.is_modified());
    }
    fn test_error(semantics: CursorSemantics){
        let mut doc = Document::new(semantics);
        assert!(undo::document_impl(&mut doc, semantics).is_err());
    }

    #[test] fn with_insert_change_on_stack(){
        test(
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(9, 10), Direction::Forward)], 0, 
            vec![
                ChangeSet::new(
                    vec![Change::new(
                        Operation::Insert{inserted_text: "some\n".to_string()}, 
                        Selection::new(Range::new(4, 5), Direction::Forward), 
                        Selection::new(Range::new(9, 10), Direction::Forward), 
                        Operation::Delete
                    )], 
                    Selections::new(vec![Selection::new(Range::new(4, 5), Direction::Forward)], 0, &Rope::from("idk\nshit\n"), CursorSemantics::Block), 
                    Selections::new(vec![Selection::new(Range::new(9, 10), Direction::Forward)], 0, &Rope::from("idk\nsome\nshit\n"), CursorSemantics::Block)
                )
            ], 
            "idk\nshit\n", 
            "idk\nshit\n", 
            vec![Selection::new(Range::new(4, 5), Direction::Forward)], 0, 
            CursorSemantics::Block
        );
    }

    //TODO: test with delete_change_on_stack
    //TODO: test with replace change on stack
    //TODO: test with no_op change on stack

    //TODO: test with multiple selections/changes

    #[test] fn undo_with_nothing_on_stack_errors(){
        test_error(CursorSemantics::Block);
        test_error(CursorSemantics::Bar);
    }
}
