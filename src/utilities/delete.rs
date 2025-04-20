use crate::{
    document::{Document, DocumentError},
    selection::CursorSemantics,
    history::{ChangeSet, Change, Operation},
};

//TODO: can this function and backspace be combined?...

/// Deletes text inside each [`Selection`] in [`Selections`], or if [`Selection`] not extended, the next character, and pushes changes to undo stack.
pub fn document_impl(document: &mut Document, semantics: CursorSemantics) -> Result<(), DocumentError>{
    let selections_before_changes = document.selections.clone();
    let mut changes = Vec::new();
    let mut cannot_delete = false;
    for i in 0..document.selections.count(){
        let selection = document.selections.nth_mut(i);
        //handles cursor at doc end
        if selection.anchor() == document.text.len_chars() && selection.cursor(&document.text, semantics) == document.text.len_chars(){
            cannot_delete = true; //don't modify text buffer here...
            let change = Change::new(Operation::NoOp, selection.clone(), selection.clone(), Operation::NoOp);
            changes.push(change);
        }
        else{   //apply the delete
            let change = Document::apply_delete(&mut document.text, selection, semantics);
            if let Operation::Insert{inserted_text} = change.inverse(){
                document.selections.shift_subsequent_selections_backward(i, inserted_text.len());
            }
            changes.push(change);
        }
    }

    if document.selections.count() == 1 && cannot_delete{return Err(DocumentError::SelectionAtDocBounds);}
    else{
        // push change set to undo stack
        document.undo_stack.push(ChangeSet::new(changes, selections_before_changes, document.selections.clone()));

        // clear redo stack. new actions invalidate the redo history
        document.redo_stack.clear();
    }

    Ok(())
}

#[cfg(test)]
mod tests{
    use crate::utilities::delete;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics, Direction},
        range::Range,
    };
    use ropey::Rope;

    fn test(text: &str, selections: Vec<Selection>, primary: usize, expected_text: &str, expected_selections: Vec<Selection>, expected_primary: usize, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        let _ = delete::document_impl(&mut doc, semantics);
        let expected_text = Rope::from(expected_text);
        assert_eq!(expected_text, doc.text);
        let expected_selections = Selections::new(expected_selections, expected_primary, &text);
        assert_eq!(expected_selections, doc.selections);
        assert!(doc.is_modified());
    }
    fn test_error(text: &str, selections: Vec<Selection>, primary: usize, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        assert!(delete::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    #[test] fn with_non_extended_selections(){
        test(
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(4, 5), Direction::Forward)
            ], 0, 
            "dk\nome\nshit\n", 
            vec![
                Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0),
                Selection::with_stored_line_position(Range::new(3, 4), Direction::Forward, 0)
            ], 0, 
            CursorSemantics::Block
        );
    }

    #[test] fn with_extended_selections(){
        test(
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 2), Direction::Forward),
                Selection::new(Range::new(4, 6), Direction::Forward)
            ], 0, 
            "k\nme\nshit\n", 
            vec![
                Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0),
                Selection::with_stored_line_position(Range::new(2, 3), Direction::Forward, 0)
            ], 0, 
            CursorSemantics::Block
        );
    }

    #[test] fn with_valid_selection_and_cursor_at_doc_end(){
        test(
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(9, 10), Direction::Forward),
                Selection::new(Range::new(14, 15), Direction::Forward)
            ], 0, 
            "idk\nsome\nhit\n", 
            vec![
                Selection::with_stored_line_position(Range::new(9, 10), Direction::Forward, 0),
                Selection::new(Range::new(13, 14), Direction::Forward)
            ], 0, 
            CursorSemantics::Block
        );
    }

    #[test] fn errors_if_single_cursor_at_doc_end(){
        test_error(
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(14, 15), Direction::Forward)], 0, 
            CursorSemantics::Block
        );
    }
}
