use crate::{
    document::{Document, DocumentError},
    selection::CursorSemantics,
    history::{ChangeSet, Change, Operation},
    text_util,
};

//TODO: combine backspace with delete (make delete take a direction::Forward/Backward)

/// Deletes the previous character, or deletes selection if extended.
/// #### Invariants:
/// - will not delete past start of doc
/// - at start of line, appends current line to end of previous line
/// - removes previous soft tab, if `TAB_WIDTH` spaces are before cursor
/// - deletes selection if selection extended
pub fn document_impl(document: &mut Document, use_hard_tab: bool, tab_width: usize, semantics: CursorSemantics) -> Result<(), DocumentError>{
    let selections_before_changes = document.selections.clone();
    let mut changes = Vec::with_capacity(document.selections.count());
    let mut cannot_delete = false;

    for i in 0..document.selections.count(){
        let selection = document.selections.nth_mut(i);
        if selection.is_extended(semantics){
            let change = Document::apply_delete(&mut document.text, selection, semantics);
            if let Operation::Insert{inserted_text} = change.inverse(){
                document.selections.shift_subsequent_selections_backward(i, inserted_text.len());
            }
            changes.push(change);
        }else{
            if selection.anchor() == 0 && selection.cursor(&document.text, semantics) == 0{
                cannot_delete = true; //don't modify text buffer here...
                let change = Change::new(Operation::NoOp, selection.clone(), selection.clone(), Operation::NoOp);
                changes.push(change);
            }
            else{
                let offset_from_line_start = text_util::offset_from_line_start(selection.cursor(&document.text, semantics), &document.text);
                let line = document.text.line(document.text.char_to_line(selection.cursor(&document.text, semantics)));
                let is_deletable_soft_tab = !use_hard_tab && offset_from_line_start >= tab_width
                // handles case where user adds a space after a tab, and wants to delete only the space
                && offset_from_line_start % tab_width == 0
                // if previous 4 chars are spaces, delete 4. otherwise, use default behavior
                && text_util::slice_is_all_spaces(line.slice(offset_from_line_start.saturating_sub(tab_width)..offset_from_line_start));

                if is_deletable_soft_tab{
                    selection.shift_and_extend(tab_width, &document.text, semantics);
                    changes.push(Document::apply_delete(&mut document.text, selection, semantics));
                    document.selections.shift_subsequent_selections_backward(i, tab_width);
                }
                else{
                    //if let Ok(new_selection) = selection.move_left(&document.text, semantics){
                    if let Ok(new_selection) = crate::utilities::move_cursor_left::selection_impl(selection, &document.text, semantics){
                        *selection = new_selection;
                    }   //TODO: handle error    //first for loop guarantees no selection is at doc bounds, so this should be ok to ignore...
                    changes.push(Document::apply_delete(&mut document.text, selection, semantics));
                    document.selections.shift_subsequent_selections_backward(i, 1);
                }
            }
        }
    }

    if document.selections.count() == 1 && cannot_delete{return Err(DocumentError::SelectionAtDocBounds);}
    else{
        // push changes to undo stack
        document.undo_stack.push(ChangeSet::new(changes, selections_before_changes, document.selections.clone()));

        // clear redo stack. new actions invalidate the redo history
        document.redo_stack.clear();
    }

    Ok(())
}

#[cfg(test)]
mod tests{
    use crate::utilities::backspace;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics, Direction},
        range::Range,
    };
    use ropey::Rope;

    fn test(semantics: CursorSemantics, text: &str, selections: Vec<Selection>, primary: usize, expected_text: &str, expected_selections: Vec<Selection>, expected_primary: usize){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        let result = backspace::document_impl(&mut doc, false, 4, semantics);
        assert!(!result.is_err());
        let expected_text = Rope::from(expected_text);
        assert_eq!(expected_text, doc.text);
        let expected_selections = Selections::new(expected_selections, expected_primary, &text);
        assert_eq!(expected_selections, doc.selections);
        assert!(doc.is_modified());
    }
    fn test_error(semantics: CursorSemantics, text: &str, selections: Vec<Selection>, primary: usize){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        assert!(backspace::document_impl(&mut doc, false, 4, semantics).is_err());
        assert!(!doc.is_modified());
    }

    #[test] fn common_use(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(1, 2), Direction::Forward),
                Selection::new(Range::new(5, 6), Direction::Forward)
            ], 0, 
            "dk\nome\nshit\n",
            vec![
                Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0),
                Selection::with_stored_line_position(Range::new(3, 4), Direction::Forward, 0)
            ], 0
        );
    }

    #[test] fn when_at_line_start_appends_current_line_to_previous_line(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(4, 5), Direction::Forward)], 0, 
            "idksome\nshit\n", 
            vec![Selection::with_stored_line_position(Range::new(3, 4), Direction::Forward, 3)], 0
        );
    }

    #[test] fn with_valid_selection_and_cursor_at_doc_start(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(4, 5), Direction::Forward)
            ], 0, 
            "idksome\nshit\n",
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::with_stored_line_position(Range::new(3, 4), Direction::Forward, 3)
            ], 0
        );
    }

    #[test] fn errors_if_single_cursor_at_doc_start(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0
        );
    }

    #[test] fn with_extended_selection_deletes_selection(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(0, 4), Direction::Forward)
            ], 0, 
            "some\nshit\n",
            vec![
                Selection::with_stored_line_position(Range::new(0, 1), Direction::Forward, 0)
            ], 0
        );
    }

    //TODO: test tab deletion with soft tabs
    //TODO: test tab deletion with hard tabs
    //TODO: test with various tab widths
}
