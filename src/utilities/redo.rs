use crate::{
    document::{Document, DocumentError},
    selection::CursorSemantics,
    history::Operation
};
use std::cmp::Ordering;

/// Re-applies the last undone changes to the document.
// Make sure to clear the redo stack in every edit fn. new actions invalidate the redo history
pub fn document_impl(document: &mut Document, semantics: CursorSemantics) -> Result<(), DocumentError>{    //should this be HistoryError instead?...
    // Check if there is something to redo
    if let Some(change_set) = document.redo_stack.pop(){
        let changes = change_set.changes();

        document.selections = change_set.clone().selections_before_changes();    //set selections to selections_before_changes to account for any selection movements that may have occurred since undo
        assert!(document.selections.count() == changes.len());   //num selections should match num changes

        for (i, change) in changes.iter().enumerate().take(document.selections.count()){
            let selection = document.selections.nth_mut(i);
            match change.operation(){
                Operation::Insert{inserted_text} => {
                    let _ = Document::apply_insert(&mut document.text, &inserted_text, selection, semantics);
                    document.selections.shift_subsequent_selections_forward(i, inserted_text.len());
                }
                Operation::Delete => {
                    *selection = change.selection_before_change();
                    let change = Document::apply_delete(&mut document.text, selection, semantics);
                    if let Operation::Insert{inserted_text} = change.inverse(){
                        document.selections.shift_subsequent_selections_backward(i, inserted_text.len());
                    }
                }
                Operation::Replace{replacement_text} => {
                    let inserted_text = replacement_text;
                    let change = Document::apply_replace(&mut document.text, &inserted_text, selection, semantics);
                    if let Operation::Replace{replacement_text} = change.inverse(){   //destructure to get currently selected text
                        match replacement_text.len().cmp(&inserted_text.len()){    //old selected text vs new text
                            Ordering::Greater => {document.selections.shift_subsequent_selections_backward(i, replacement_text.len().saturating_sub(inserted_text.len()));}
                            Ordering::Less => {document.selections.shift_subsequent_selections_forward(i, inserted_text.len().saturating_sub(replacement_text.len()));}
                            Ordering::Equal => {}   // no change to subsequent selections
                        }
                    }
                }
                Operation::NoOp => {}
            }
        }
        assert!(document.selections == change_set.clone().selections_after_changes());

        // Push changes back onto the undo stack
        document.undo_stack.push(change_set);

        Ok(())
    }else{Err(DocumentError::NoChangesToRedo)}
}

#[cfg(test)]
mod tests{
    use crate::utilities::redo;
    use crate::document::Document;
    use crate::selection::CursorSemantics;

    //TODO: test more

    #[test] fn redo_with_nothing_on_stack_errors(){
        let mut doc = Document::new(CursorSemantics::Bar);
        assert!(redo::document_impl(&mut doc, CursorSemantics::Bar).is_err());
    }
}