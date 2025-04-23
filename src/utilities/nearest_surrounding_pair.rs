use crate::{
    document::{Document, DocumentError},
    selections::{Selections, SelectionsError},
    selection::{Selection, CursorSemantics, Direction},
    range::Range,
    text_util
};
use ropey::Rope;


pub fn document_impl(document: &mut Document, semantics: CursorSemantics) -> Result<(), DocumentError>{
    match selections_impl(&document.selections, &document.text, semantics){
        Ok(new_selections) => {document.selections = new_selections;}
        Err(e) => {return Err(DocumentError::SelectionsError(e));}
    }
    Ok(())
}

//TODO: for some reason, repeated calls after successfully selecting bracket pair do not return same state error...
fn selections_impl(selections: &Selections, text: &Rope, semantics: CursorSemantics) -> Result<Selections, SelectionsError>{
    let mut new_selections = Vec::with_capacity(2 * selections.count());
    let mut num_pushed: usize = 0;
    let primary_selection = selections.primary();
    let mut primary_selection_index = selections.primary_selection_index;
    for selection in &selections.selections{
        let surrounds = selection_impl(selection, text);
        if selection == primary_selection{
            primary_selection_index = num_pushed;
        }
        if surrounds.is_empty(){//push selection
            new_selections.push(selection.clone());
            num_pushed = num_pushed + 1;
        }
        else{//push surrounds
            for surround in surrounds{
                new_selections.push(surround);
                num_pushed = num_pushed + 1;
            }
        }
    }
    if new_selections.is_empty() || new_selections == selections.selections{Err(SelectionsError::ResultsInSameState)}
    else{
        //Ok(Selections::new(new_selections, primary_selection_index, text))
        Selections::new(new_selections, primary_selection_index, text).sort().merge_overlapping(text, semantics)
    }
}

//TODO: maybe this should be implemented with treesitter, so irrelevant pairs(like ' characters inside words(like don't)) aren't matched
//TODO: maybe front end should pass in their view of what is a valid surrounding pair, then we can match those...to make this as flexible as possible
//TODO: think about how surrounding quotation pairs should be handled
/// Returns a new pair of [`Selection`]s with each selection over the nearest surrounding grapheme pair, if possible
/// valid pairs:    //maybe add ':', '*'
/// { }
/// ( )
/// [ ]
/// < >
/// ' '
/// " "
#[must_use] fn selection_impl(selection: &Selection, text: &Rope) -> Vec<Selection>{
    let mut rev_search_index = selection.range.start;
    'outer: loop{
        let current_char = text.char(rev_search_index);
        if is_opening_bracket(current_char){
            let opening_char = current_char;
            let closing_char = get_matching_closing_bracket(opening_char);
            let mut match_stack = Vec::new();
            let mut search_index = rev_search_index;
            'inner: loop{
                let current_char = text.char(search_index);
                if opening_char == closing_char{  //search before cursor for previous instance of char, then after cursor for next instance. ignore hierarchy because i'm not sure we can parse that...
                    if current_char == closing_char{
                        if match_stack.is_empty(){
                            match_stack.push(current_char);
                        }
                        else{
                            return vec![
                                Selection::new(Range::new(rev_search_index, text_util::next_grapheme_index(rev_search_index, text)), Direction::Forward),
                                Selection::new(Range::new(search_index, text_util::next_grapheme_index(search_index, text)), Direction::Forward)
                            ];
                        }
                    }
                    else{/*do nothing. index will be incremented below...*/}
                }
                else{
                    if current_char == opening_char{
                        match_stack.push(current_char);
                    }
                    else if current_char == closing_char{
                        match_stack.pop();
                        if match_stack.is_empty(){
                            if search_index >= selection.range.start{
                                return vec![
                                    Selection::new(Range::new(rev_search_index, text_util::next_grapheme_index(rev_search_index, text)), Direction::Forward),
                                    Selection::new(Range::new(search_index, text_util::next_grapheme_index(search_index, text)), Direction::Forward)
                                ];
                            }
                            else{break 'inner;}
                        }
                        else{/*do nothing. index will be incremented below...*/}
                    }
                }
                    
                search_index = search_index + 1;

                if search_index >= text.len_chars(){break 'outer;}
            }
        }
        //else{ //is else really needed here?...
            rev_search_index = rev_search_index.saturating_sub(1);
        //}

        if rev_search_index == 0{break 'outer;}
    }

    Vec::new()
}
fn is_opening_bracket(char: char) -> bool{  //TODO: this should prob be in text_util.rs
    char == '{'
    || char == '('
    || char == '['
    || char == '<'
    || char == '\''
    || char == '"'
}
fn get_matching_closing_bracket(char: char) -> char{    //TODO: this should prob be in text_util.rs
    if char == '{'{'}'}
    else if char == '('{')'}
    else if char == '['{']'}
    else if char == '<'{'>'}
    else if char == '\''{'\''}
    else if char == '"'{'"'}
    else{panic!();} //TODO: maybe return None, or an error?...
}

#[cfg(test)]
mod tests{
    use crate::utilities::nearest_surrounding_pair;
    use crate::{
        document::Document,
        selections::Selections,
        selection::{Selection, CursorSemantics, Direction},
        range::Range,
    };
    use ropey::Rope;

    fn test(semantics: CursorSemantics, text: &str, selections: Vec<Selection>, primary: usize, expected_selections: Vec<Selection>, expected_primary: usize){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        let result = nearest_surrounding_pair::document_impl(&mut doc, semantics);
        assert!(!result.is_err());
        let expected_selections = Selections::new(expected_selections, expected_primary, &text);
        assert_eq!(expected_selections, doc.selections);
        assert!(!doc.is_modified());
    }
    fn test_error(semantics: CursorSemantics, text: &str, selections: Vec<Selection>, primary: usize){
        let text = Rope::from(text);
        let mut doc = Document::new(semantics)
            .with_text(text.clone())
            .with_selections(Selections::new(selections, primary, &text));
        assert!(nearest_surrounding_pair::document_impl(&mut doc, semantics).is_err());
        assert!(!doc.is_modified());
    }

    #[test] fn with_multiple_selections(){
        //                     1                   2
        // 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7
        //|i>d k ( s|o>m e|[>] _ t h|i>n g _ {|}>e l s e ) _ i|d>k
        //|i>d k|(>s o m e|[>]>_ t h i n g _|{>}>e l s e|)>_ i|d>k
        test(
            CursorSemantics::Block, 
            "idk(some[] thing {}else) idk", 
            vec![
                Selection::new(Range::new(0, 1), Direction::Forward),   //no pair
                Selection::new(Range::new(5, 6), Direction::Forward),   //pair
                Selection::new(Range::new(8, 9), Direction::Forward),   //pair
                Selection::new(Range::new(13, 14), Direction::Forward), //pair
                Selection::new(Range::new(18, 19), Direction::Forward), //pair
                Selection::new(Range::new(26, 27), Direction::Forward), //no pair
            ], 0, 
            vec![
                //same as above, but sorted and merged, if needed
                Selection::new(Range::new(0, 1), Direction::Forward),
                Selection::with_stored_line_position(Range::new(3, 4), Direction::Forward, 3),      //idk why these have stored line position and others don't
                Selection::new(Range::new(8, 9), Direction::Forward),
                Selection::new(Range::new(9, 10), Direction::Forward),
                Selection::new(Range::new(17, 18), Direction::Forward),
                Selection::new(Range::new(18, 19), Direction::Forward),
                Selection::with_stored_line_position(Range::new(23, 24), Direction::Forward, 23),   //idk why these have stored line position and others don't
                Selection::new(Range::new(26, 27), Direction::Forward),
                //TODO: merge overlapping in selection.rs causing the stored line position. only the overlapping selections have it
                //if so, this should def be fixed in merge_overlapping impl
                //or more correctly, every movement fn should update the stored line position...
                    //the only reason we have a None variant is so that we don't need to take a &Rope in Selection::new()
            ], 0
        );
    }

    ////|i>d k ( s o m e [ ] _ t h i n g _ { } e l s e ) _ i d k     //no surrounding pair with cursor at this location
    #[test] fn at_start_with_no_surrounding_pair(){
        test_error(
            CursorSemantics::Block, 
            "idk(some[] thing {}else) idk", 
            vec![Selection::new(Range::new(0, 1), Direction::Forward)], 0
        );
    }

    //// i d k ( s|o>m e [ ] _ t h i n g _ { } e l s e ) _ i d k     //paren surrounding pair with cursor at this location
    #[test] fn normal_case(){
        test(
            CursorSemantics::Block, 
            "idk(some[] thing {}else) idk", 
            vec![Selection::new(Range::new(5, 6), Direction::Forward)], 0, 
            vec![
                Selection::new(Range::new(3, 4), Direction::Forward),
                Selection::new(Range::new(23, 24), Direction::Forward)
            ], 0
        );
    }

    //// i d k ( s o m e|[>] _ t h i n g _ { } e l s e ) _ i d k     //square bracket surrounding pair with cursor at this location
    #[test] fn with_cursor_over_surrounding_pair_opening(){
        test(
            CursorSemantics::Block, 
            "idk(some[] thing {}else) idk", 
            vec![Selection::new(Range::new(8, 9), Direction::Forward)], 0, 
            vec![
                Selection::new(Range::new(8, 9), Direction::Forward),
                Selection::new(Range::new(9, 10), Direction::Forward)
            ], 0
        );
    }

    //// i d k ( s o m e [ ] _ t h|i>n g _ { } e l s e ) _ i d k     //paren surrounding pair with cursor at this location
    #[test] fn with_other_pairs_inside_surrounding_pair(){
        test(
            CursorSemantics::Block, 
            "idk(some[] thing {}else) idk", 
            vec![Selection::new(Range::new(13, 14), Direction::Forward)], 0, 
            vec![
                Selection::new(Range::new(3, 4), Direction::Forward),
                Selection::new(Range::new(23, 24), Direction::Forward)
            ], 0
        );
    }

    //// i d k ( s o m e [ ] _ t h i n g _ {|}>e l s e ) _ i d k     //curly bracket surrounding pair with cursor at this location
    #[test] fn with_cursor_over_surrounding_pair_closing(){
        test(
            CursorSemantics::Block, 
            "idk(some[] thing {}else) idk", 
            vec![Selection::new(Range::new(18, 19), Direction::Forward)], 0, 
            vec![
                Selection::new(Range::new(17, 18), Direction::Forward),
                Selection::new(Range::new(18, 19), Direction::Forward)
            ], 0
        );
    }

    //// i d k ( s o m e [ ] _ t h i n g _ { } e l s e ) _ i|d>k     //no surrounding pair with cursor at this location
    #[test] fn at_end_with_no_surrounding_pair(){
        test_error(
            CursorSemantics::Block, 
            "idk(some[] thing {}else) idk", 
            vec![Selection::new(Range::new(26, 27), Direction::Forward)], 0
        );
    }

    //These two seem redundant given previous tests...
    #[test] fn no_opening_bracket_pair_returns_empty_vec(){
        test_error(
            CursorSemantics::Block, 
            "idk\nsomething)\n", 
            vec![Selection::new(Range::new(3, 4), Direction::Forward)], 0
        );
    }
    #[test] fn no_closing_bracket_pair_returns_empty_vec(){
        test_error(
            CursorSemantics::Block, 
            "(idk\nsomething\n", 
            vec![Selection::new(Range::new(3, 4), Direction::Forward)], 0
        );
    }

    ////idk(some()t(h(i)n)g()else)    //test from multiple levels of same surrounding pair
    #[test] fn with_multiple_levels_of_same_surrounding_pair(){
        test(
            CursorSemantics::Block, 
            "idk(some()t(h(i)n)g()else", 
            vec![Selection::new(Range::new(12, 13), Direction::Forward)], 0, 
            vec![
                Selection::new(Range::new(11, 12), Direction::Forward),
                Selection::new(Range::new(17, 18), Direction::Forward)
            ], 0
        );
    }

    //TODO: impl test with expected quote pair behavior
    //note: quote pairs may have to work differently than bracket pairs
    //#[test] fn with_same_surrounding_pair_opening_and_closing(){
    //    //idk"some""t"h"i"n"g""else"
    //    let text = Rope::from("idk\"some\"\"t\"h\"i\"n\"g\"\"else");
    //    let selection = Selection::new(Range::new(12, 13), Direction::Forward);
    //    assert_eq!(
    //        vec![
    //            Selection::new(Range::new(11, 12), Direction::Forward),
    //            //Selection::new(Range::new(17, 18), Direction::Forward)
    //            Selection::new(Range::new(13, 14), Direction::Forward)
    //        ],
    //        selection.nearest_surrounding_pair(&text)
    //    );
    //}
}
