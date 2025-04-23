use crate::{
    document::{Document, DocumentError},
    selections::{Selections, SelectionsError},
    selection::{Selection, Direction, CursorSemantics},
    range::Range
};
use ropey::Rope;
use regex::Regex;

pub fn document_impl(document: &mut Document, search_text: &str, selections_before_search: &Selections, semantics: CursorSemantics) -> Result<(), DocumentError>{
    match selections_impl(selections_before_search, search_text, &document.text, semantics){
        Ok(new_selections) => {
            document.selections = new_selections;
            Ok(())
        }
        Err(_) => {
            document.selections = selections_before_search.clone();
            Err(DocumentError::InvalidInput)
        }
    }
}

//TODO: maybe. if no selection extended, search whole text
/// 
/// # Errors
///     //if no matches.
pub fn selections_impl(selections: &Selections, input: &str, text: &Rope, semantics: CursorSemantics) -> Result<Selections, SelectionsError>{
    if input.is_empty(){return Err(SelectionsError::NoSearchMatches);}
    let mut new_selections = Vec::new();
    let mut num_pushed: usize = 0;
    let primary_selection = selections.primary();
    //let mut primary_selection_index = self.primary_selection_index;
    let mut primary_selection_index = 0;
    
    for selection in &selections.selections{  //self.selections.iter(){   //change suggested by clippy lint
        let matches = selection_impl(selection, input, text);
        if selection == primary_selection{
            primary_selection_index = num_pushed.saturating_sub(1);
        }
        for search_match in matches{
            new_selections.push(search_match);
            num_pushed = num_pushed + 1;
        }
    }

    if new_selections.is_empty(){Err(SelectionsError::NoSearchMatches)}
    else{
        Ok(Selections::new(new_selections, primary_selection_index, text, semantics))
    }
}

/// Returns a [`Vec`] of [`Selection`]s where the underlying text is a match for the `input` search string.
#[must_use] pub fn selection_impl(selection: &Selection, input: &str, text: &Rope) -> Vec<Selection>{   //text should be the text within a selection, not the whole document text       //TODO: -> Result<Vec<Selection>>
    let mut selections = Vec::new();
    let start = selection.range.start;

    //match Regex::new(input){
    //    Ok(regex) => {
    //        for search_match in regex.find_iter(&text.to_string()[start..self.range.end.min(text.len_chars())]){
    //            selections.push(Selection::new(search_match.start().saturating_add(start), search_match.end().saturating_add(start)));
    //        }
    //    }
    //    Err(_) => {}    //return error FailedToParseRegex
    //}
    if let Ok(regex) = Regex::new(input){
        for search_match in regex.find_iter(&text.to_string()[start..selection.range.end.min(text.len_chars())]){
            //selections.push(Selection::new(search_match.start().saturating_add(start), search_match.end().saturating_add(start)));
            selections.push(Selection::new(Range::new(search_match.start().saturating_add(start), search_match.end().saturating_add(start)), Direction::Forward));
        }
    }
    //else{/*return error FailedToParseRegex*/}

    if selections.is_empty(){
        //return NoMatch error      //this error is not strictly necessary since caller can just check for an empty return vec
    }
    selections
}

#[cfg(test)]
mod tests{
    use crate::utilities::incremental_search_in_selection;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics, Direction},
        range::Range,
    };
    use ropey::Rope;

    fn test(semantics: CursorSemantics, text: &str, search_text: &str, selections: Vec<Selection>, primary: usize, expected_selections: Vec<Selection>, expected_primary: usize){
        let text = Rope::from(text);
        let selections = Selections::new(selections, primary, &text, semantics);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(selections.clone());
        let result = incremental_search_in_selection::document_impl(&mut doc, search_text, &selections, semantics);
        assert!(!result.is_err());
        let expected_selections = Selections::new(expected_selections, expected_primary, &text, semantics);
        assert_eq!(expected_selections, doc.selections);
        assert!(!doc.is_modified());
    }
    fn test_error(semantics: CursorSemantics, text: &str, search_text: &str, selections: Vec<Selection>, primary: usize){
        let text = Rope::from(text);
        let selections = Selections::new(selections, primary, &text, semantics);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(selections.clone());
        assert!(incremental_search_in_selection::document_impl(&mut doc, search_text, &selections, semantics).is_err());
        assert!(!doc.is_modified());
    }

    //multiple selections on same line      + primary updates as expected
    #[test] fn with_multiple_selections_on_same_line(){
        test(
            CursorSemantics::Block, 
            "idk some shit", 
            "s[oh]", 
            vec![
                Selection::new(Range::new(0, 3), Direction::Forward),   //expect no match
                Selection::new(Range::new(4, 8), Direction::Forward),   //expect match
                Selection::new(Range::new(9, 13), Direction::Forward)   //expect match
            ], 2, 
            vec![
                Selection::new(Range::new(4, 6), Direction::Forward),
                Selection::new(Range::new(9, 11), Direction::Forward)
            ], 0    //is this correct?...   doesn't seem right
        );
    }
    #[test] fn with_multiple_selections_on_different_lines(){
        test(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            "s[oh]", 
            vec![
                Selection::new(Range::new(0, 3), Direction::Forward),   //expect no match
                Selection::new(Range::new(4, 8), Direction::Forward),   //expect match
                Selection::new(Range::new(9, 13), Direction::Forward)   //expect match
            ], 2, 
            vec![
                Selection::new(Range::new(4, 6), Direction::Forward),
                Selection::new(Range::new(9, 11), Direction::Forward)
            ], 0    //is this correct?...
        );
    }
    #[test] fn errors_if_all_selections_have_no_match(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            "x", 
            vec![
                Selection::new(Range::new(0, 3), Direction::Forward),   //expect no match
                Selection::new(Range::new(4, 8), Direction::Forward),   //expect no match
                Selection::new(Range::new(9, 13), Direction::Forward)   //expect no match
            ], 0
        );
    }
}
