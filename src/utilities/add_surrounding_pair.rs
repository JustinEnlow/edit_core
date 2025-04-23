use crate::{
    document::{Document, DocumentError},
    selection::CursorSemantics,
    history::{ChangeSet, Change, Operation},
};

//TODO: i think all edit actions + apply replace/insert/delete should prob be made purely functional...

//had to make the following public
    //Document.text
    //Document.selections
    //Document.undo_stack
    //Document.redo_stack
    //Document::apply_replace
//is this easing of encapsulation acceptable?...
pub fn document_impl(document: &mut Document, leading_char: char, trailing_char: char, semantics: CursorSemantics) -> Result<(), DocumentError>{
    let selections_before_changes = document.selections.clone();
    let mut changes = Vec::new();
    let mut cannot_add_surrounding_pair = false;  //to handle cursor at doc end...
    for i in 0..document.selections.count(){
        let selection = document.selections.nth_mut(i);
        //handles cursor at doc end
        if selection.anchor() == document.text.len_chars() && selection.cursor(&document.text, semantics) == document.text.len_chars(){
            cannot_add_surrounding_pair = true; //don't modify text buffer here...
            let change = Change::new(Operation::NoOp, selection.clone(), selection.clone(), Operation::NoOp);
            changes.push(change);
        }
        else{   //replace each selection with its text contents + leading and trailing char added
            let mut contents = selection.contents_as_string(&document.text);
            contents.insert(0, leading_char);
            contents.push(trailing_char);
            let change = Document::apply_replace(&mut document.text, &contents, selection, CursorSemantics::Block);
            changes.push(change);
            document.selections.shift_subsequent_selections_forward(i, 2);  //TODO: could this be handled inside apply_replace and similar functions?...
        }
    }

    if document.selections.count() == 1 && cannot_add_surrounding_pair{return Err(DocumentError::SelectionAtDocBounds);}
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
    use crate::utilities::add_surrounding_pair;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{CursorSemantics, Direction, Selection}, 
        range::Range, 
    };
    use ropey::Rope;

    fn test(text: &str, selections: Vec<Selection>, primary: usize, leading_char: char, trailing_char: char, expected_text: &str, expected_selections: Vec<Selection>, expected_primary: usize, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        let _ = add_surrounding_pair::document_impl(&mut doc, leading_char, trailing_char, semantics);
        let expected_text = Rope::from(expected_text);
        assert_eq!(expected_text, doc.text);
        let expected_selections = Selections::new(expected_selections, expected_primary, &text);
        assert_eq!(expected_selections, doc.selections);
        assert!(doc.is_modified());
    }
    fn test_error(text: &str, selections: Vec<Selection>, primary: usize, leading_char: char, trailing_char: char, semantics: CursorSemantics){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        assert!(add_surrounding_pair::document_impl(&mut doc, leading_char, trailing_char, semantics).is_err());
        assert!(!doc.is_modified());
    }

    #[test] fn with_single_selection(){
        test(
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(0, 3), Direction::Forward)], 0, 
            '{', '}', 
            "{idk}\nsome\nshit\n", 
            vec![Selection::with_stored_line_position(Range::new(5, 6), Direction::Forward, 5)], 0, 
            CursorSemantics::Block
        );
    }

    //TODO: test multiple selections

    //TODO: test with selection over newline(should be the same, but worth verifying...)

    #[test] fn with_valid_selection_and_cursor_at_end_of_doc(){
        test(
            "idk\nsome\nshit\n", 
            vec![
                Selection::new(Range::new(9, 11), Direction::Forward),
                Selection::new(Range::new(14, 15), Direction::Forward)
            ], 0, 
            '<', '>', 
            "idk\nsome\n<sh>it\n", 
            vec![
                Selection::with_stored_line_position(Range::new(13, 14), Direction::Forward, 4),
                Selection::new(Range::new(16, 17), Direction::Forward)
            ], 0, 
            CursorSemantics::Block
        );
    }

    #[test] fn errors_when_single_cursor_at_end_of_document(){
        test_error(
            "idk\nsome\nshit\n", 
            vec![Selection::new(Range::new(14, 15), Direction::Forward)], 0, 
            '{', '}', 
            CursorSemantics::Block
        );
    }

    //TODO?: should resultant selection after adding surrounding pair be a selection over the content and pair?...
    //i think this is a much deeper question than this single function...
    //this relates to all replacement text  (if we use the default Document::apply_replace...)
}
