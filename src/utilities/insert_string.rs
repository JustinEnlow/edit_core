use crate::{
    document::{Document, DocumentError},
    selection::CursorSemantics,
    history::{ChangeSet, Change, Operation},
    text_util
};

/// Inserts provided string into text at each selection.
pub fn document_impl(document: &mut Document, string: &str, use_hard_tab: bool, tab_width: usize, semantics: CursorSemantics) -> Result<(), DocumentError>{
    let selections_before_changes = document.selections.clone();
    let mut changes = Vec::new();

    if string.is_empty(){return Err(DocumentError::InvalidInput);}

    for i in 0..document.selections.count(){
        let selection = document.selections.nth_mut(i);
        let change = match string{
            //"\n" => {}    //handle behavior specific to pressing "enter". auto-indent, etc... //TODO: create tests for newline behavior...
            "\t" => {   //handle behavior specific to pressing "tab".
                if use_hard_tab{
                    if selection.is_extended(semantics){handle_insert_replace(document, i, semantics, "\t")}
                    else{handle_insert(document, "\t", i, semantics)}
                }
                else{
                    let tab_distance = text_util::distance_to_next_multiple_of_tab_width(selection, &document.text, semantics);
                    let modified_tab_width = if tab_distance > 0 && tab_distance < tab_width{tab_distance}else{tab_width};
                    let soft_tab = " ".repeat(modified_tab_width);

                    if selection.is_extended(semantics){handle_insert_replace(document, i, semantics, &soft_tab)}
                    else{handle_insert(document, &soft_tab, i, semantics)}
                }
            }
            //handle any other inserted string
            _ => {
                if selection.is_extended(semantics){handle_insert_replace(document, i, semantics, string)}
                else{handle_insert(document, string, i, semantics)}
            }
        };

        changes.push(change);
    }

    // push change set to undo stack
    document.undo_stack.push(ChangeSet::new(changes, selections_before_changes, document.selections.clone()));

    // clear redo stack. new actions invalidate the redo history
    document.redo_stack.clear();

    Ok(())
}
fn handle_insert_replace(document: &mut Document, current_selection_index: usize, semantics: CursorSemantics, new_text: &str) -> Change{
    use std::cmp::Ordering;
    let selection = document.selections.nth_mut(current_selection_index);
    let change = Document::apply_replace(&mut document.text, new_text, selection, semantics);
    if let Operation::Replace{replacement_text} = change.inverse(){
        match replacement_text.len().cmp(&new_text.len()){    //old selected text vs new text
            Ordering::Greater => {document.selections.shift_subsequent_selections_backward(current_selection_index, replacement_text.len().saturating_sub(new_text.len()));}
            Ordering::Less => {document.selections.shift_subsequent_selections_forward(current_selection_index, new_text.len().saturating_sub(replacement_text.len()));}
            Ordering::Equal => {}   // no change to subsequent selections
        }
    }
    change
}
fn handle_insert(document: &mut Document, string: &str, current_selection_index: usize, semantics: CursorSemantics) -> Change{
    let selection = document.selections.nth_mut(current_selection_index);
    let change = Document::apply_insert(&mut document.text, string, selection, semantics);
    document.selections.shift_subsequent_selections_forward(current_selection_index, string.len());
    change
}

#[cfg(test)]
mod tests{
    use crate::utilities::insert_string;
    use ropey::Rope;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics, Direction},
        range::Range,
    };

    fn test(text: &str, selections: Vec<Selection>, primary: usize, string: &str, expected_text: &str, expected_selections: Vec<Selection>, expected_primary: usize, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        let _ = insert_string::document_impl(&mut doc, string, false, 4, semantics);
        let expected_text = Rope::from(expected_text);
        assert_eq!(expected_text, doc.text);
        let expected_selections = Selections::new(expected_selections, expected_primary, &text);
        assert_eq!(expected_selections, doc.selections);
        assert!(doc.is_modified());
    }
    fn test_error(text: &str, selections: Vec<Selection>, primary: usize, string: &str, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        assert!(insert_string::document_impl(&mut doc, string, false, 4, semantics).is_err());
        assert!(!doc.is_modified());
    }

    #[test] fn insert_single_char_with_multi_selection_block_semantics(){
        test(
            "some\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(5, 6), Direction::Forward)
            ], 0, 
            "x", 
            "xsome\nxshit\n", 
            vec![
                Selection::with_stored_line_position(Range::new(1, 2), Direction::Forward, 1),
                Selection::with_stored_line_position(Range::new(7, 8), Direction::Forward, 1)
            ], 0, 
            CursorSemantics::Block
        );
    }

    #[test] fn insert_single_char_with_multi_selection_bar_semantics(){
        test(
            "some\nshit\n", 
            vec![
                Selection::new(Range::new(0, 0), Direction::Forward),
                Selection::new(Range::new(5, 5), Direction::Forward)
            ], 0, 
            "x", 
            "xsome\nxshit\n", 
            vec![
                Selection::with_stored_line_position(Range::new(1, 1), Direction::Forward, 1),
                Selection::with_stored_line_position(Range::new(7, 7), Direction::Forward, 1)
            ], 0, 
            CursorSemantics::Bar
        );
    }
    
    // TODO: insert multi-char with multi selection bar/block semantics
    //TODO: test insert tab (hard/soft/tab width)
    //TODO: test insert newline
    
    #[test] fn errors_if_empty_insert_string(){
        test_error(
            "some\nshit\n", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::new(Range::new(5, 6), Direction::Forward)
            ], 0, 
            "", 
            CursorSemantics::Block
        );
    
        test_error(
            "some\nshit\n", 
            vec![
                Selection::new(Range::new(0, 0), Direction::Forward),
                Selection::new(Range::new(5, 5), Direction::Forward)
            ], 0, 
            "", 
            CursorSemantics::Bar
        );
    }
}
