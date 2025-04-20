use crate::{
    document::{Document, DocumentError},
    selections::{Selections, SelectionsError},
    selection::{Selection, Direction},
    range::Range
};
use ropey::Rope;
use regex::Regex;

pub fn document_impl(document: &mut Document, search_text: &str, selections_before_split: &Selections, /* TODO: semantics: CursorSemantics */) -> Result<(), DocumentError>{
    match selections_impl(selections_before_split, search_text, document.text()){
        Ok(new_selections) => {
            *document.selections_mut() = new_selections;
            Ok(())
        }
        Err(_) => {
            *document.selections_mut() = selections_before_split.clone();
            Err(DocumentError::InvalidInput)
        }
    }
}

//TODO: impl tests in src/selections_tests
fn selections_impl(selections: &Selections, input: &str, text: &Rope) -> Result<Selections, SelectionsError>{
    if input.is_empty(){return Err(SelectionsError::NoSearchMatches);}
    let mut new_selections = Vec::new();
    let mut num_pushed: usize = 0;
    let primary_selection = selections.primary();
    let mut primary_selection_index = 0;
    
    for selection in selections.inner_selections(){
        let matches = selection_impl(selection, input, text);
        if matches.is_empty(){
            if selections.count() == 1{return Err(SelectionsError::NoSearchMatches);}
            if selection == primary_selection{
                primary_selection_index = num_pushed.saturating_sub(1);
            }
            new_selections.push(selection.clone());
            num_pushed = num_pushed + 1;
        }
        else{
            if selection == primary_selection{
                primary_selection_index = num_pushed.saturating_sub(1);
            }
            for search_match in matches{
                new_selections.push(search_match);
                num_pushed = num_pushed + 1;
            }
        }
    }

    let new_selections = Selections::new(new_selections, primary_selection_index, text);
    if new_selections == *selections{return Err(SelectionsError::ResultsInSameState);}

    Ok(new_selections)
}

/// Returns a [`Vec`] of [`Selection`]s containing each part of the current selection except the split pattern.
#[must_use] fn selection_impl(selection: &Selection, pattern: &str, text: &Rope) -> Vec<Selection>{
    let mut selections = Vec::new();
    if let Ok(regex) = Regex::new(pattern){
        let mut start = selection.range.start; //0;
        let mut found_split = false;
        // Iter over each split, and push the retained selection before it, if any...       TODO: test split at start of selection
        for split in regex.find_iter(&text.to_string()[selection.range.start..selection.range.end.min(text.len_chars())]){
            found_split = true;
            let selection_range = Range::new(start, split.start().saturating_add(selection.range.start));
            if selection_range.start < selection_range.end{
                //selections.push(Selection::new(selection_range.start, selection_range.end));
                selections.push(Selection::new(Range::new(selection_range.start, selection_range.end), Direction::Forward));
            }
            start = split.end().saturating_add(selection.range.start);
        }
        // Handle any remaining text after the last split
        //if split found and end of last split < selection end
        if found_split && start < selection.range.end.min(text.len_chars()){
            //selections.push(Selection::new(start, self.range.end.min(text.len_chars())));
            selections.push(Selection::new(Range::new(start, selection.range.end.min(text.len_chars())), Direction::Forward));
        }
    }
    selections
}

#[cfg(test)]
mod tests{
    use crate::utilities::incremental_split_in_selection;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics, Direction},
        range::Range,
    };
    use ropey::Rope;

    fn test(semantics: CursorSemantics, text: &str, search_text: &str, selections: Vec<Selection>, primary: usize, expected_selections: Vec<Selection>, expected_primary: usize){
        let text = Rope::from(text);
        let selections = Selections::new(selections, primary, &text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(selections.clone());
        let result = incremental_split_in_selection::document_impl(&mut doc, search_text, &selections);
        assert!(!result.is_err());
        let expected_selections = Selections::new(expected_selections, expected_primary, &text);
        assert_eq!(expected_selections, doc.selections);
        assert!(!doc.is_modified());
    }
    fn test_error(semantics: CursorSemantics, text: &str, search_text: &str, selections: Vec<Selection>, primary: usize){
        let text = Rope::from(text);
        let selections = Selections::new(selections, primary, &text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(selections.clone());
        assert!(incremental_split_in_selection::document_impl(&mut doc, search_text, &selections).is_err());
        assert_eq!(selections, doc.selections);
        assert!(!doc.is_modified());
    }

    //multiple selections on same line      + primary updates as expected
    #[test] fn with_multiple_selections_on_same_line(){
        //                    1                   2                   3
        //0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
        // i d k _ s o m e _ s h i t , _ a n d _ o t h e r , _ s t u f f
        test(
            CursorSemantics::Block, 
            "idk some shit, and other, stuff", 
            ", ", 
            vec![
                Selection::new(Range::new(0, 8), Direction::Forward),   //expect no match
                Selection::new(Range::new(9, 18), Direction::Forward),  //expect match
                Selection::new(Range::new(19, 31), Direction::Forward)  //expect match
            ], 2, 
            vec![
                Selection::new(Range::new(0, 8), Direction::Forward),   //
                Selection::new(Range::new(9, 13), Direction::Forward),
                Selection::new(Range::new(15, 18), Direction::Forward),
                Selection::new(Range::new(19, 24), Direction::Forward),
                Selection::new(Range::new(26, 31), Direction::Forward)
            ], 2    //is this correct?...   //i think this should be 3
        );
    }
    //multiple selections on different lines
    #[test] fn with_multiple_selections_on_different_lines(){
        test(
            CursorSemantics::Block, 
            "idk some\nshit, and\nother, stuff\n", 
            ", ", 
            vec![
                Selection::new(Range::new(0, 8), Direction::Forward),   //expect no match
                Selection::new(Range::new(9, 18), Direction::Forward),  //expect match
                Selection::new(Range::new(19, 31), Direction::Forward)  //expect match
            ], 0, 
            vec![
                Selection::new(Range::new(0, 8), Direction::Forward),   //
                Selection::new(Range::new(9, 13), Direction::Forward),
                Selection::new(Range::new(15, 18), Direction::Forward),
                Selection::new(Range::new(19, 24), Direction::Forward),
                Selection::new(Range::new(26, 31), Direction::Forward)
            ], 0
        );
    }
    #[test] fn errors_if_all_selections_have_no_match(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            "x", 
            vec![
                Selection::new(Range::new(0, 3), Direction::Forward),
                Selection::new(Range::new(4, 8), Direction::Forward),
                Selection::new(Range::new(9, 13), Direction::Forward)
            ], 0
        );
    }
    #[test] fn errors_if_single_selection_has_no_match(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsome\nshit\n", 
            "x", 
            vec![Selection::new(Range::new(0, 3), Direction::Forward)], 0
        );
    }
}
