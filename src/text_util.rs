use ropey::{Rope, RopeSlice};
use unicode_segmentation::UnicodeSegmentation;
use crate::document::TAB_WIDTH;
use crate::selection::{CursorSemantics, Selection};



/// Returns the count of visible graphemes in a line of text.
// TODO: handle non standard width chars such as '\t'
#[must_use] pub fn line_width(line: RopeSlice, include_newline: bool) -> usize{
    let mut line_width = 0;
    for char in line.chars(){
        if include_newline || char != '\n'{
            line_width += 1;
        }
    }
    line_width
}

//TODO: handle graphemes instead of chars?
/// Returns the offset of the first non whitespace grapheme from the start of a line of text.
#[must_use] pub fn first_non_whitespace_character_offset(line: RopeSlice) -> usize{
    let line = line.to_string();
    
    //if line.len_chars() == 0{return 0;}
    if line.is_empty(){return 0;}

    //for (index, char) in line.chars().enumerate(){
    for (index, grapheme) in line.graphemes(true).enumerate(){
        //if char != ' '{return index;}
        if grapheme != " "{return index;}
    }

    0
}

//TODO: test
#[must_use] pub fn next_grapheme_index(current_index: usize, text: &Rope) -> usize{ //should this eventually be Option<usize>?
    current_index.saturating_add(1).min(text.len_chars().saturating_add(1)) //placeholder to handle ascii text. code will need to change to handle UTF-8
}

#[must_use] pub fn previous_grapheme_index(current_index: usize, _text: &Rope) -> usize{ //should this eventually be Option<usize>?
    current_index.saturating_sub(1) //placeholder to handle ascii text. code will need to change to handle UTF-8
}

fn is_word_char(char: char) -> bool{
    if char.is_alphabetic() || char.is_numeric()/* || char == '_'*/{
        return true;
    }

    false
}

fn is_whitespace(char: char) -> bool{
    char == ' ' || char == '\t' || char == '\n'
}

/// Returns the index of the next word boundary
#[must_use] pub fn next_word_boundary(current_position: usize, text: &Rope) -> usize{   //should this be Option<usize>?
    // if current_position == text.len_chars(){return None;}
    
    let mut index = current_position;

    // Skip any leading whitespace
    while index < text.len_chars() && is_whitespace(text.char(index)){
        index = next_grapheme_index(index, text);
    }

    // Skip to end of word chars, if any
    let mut found_word_char = false;
    while index < text.len_chars() && is_word_char(text.char(index)){
        index = next_grapheme_index(index, text);
        found_word_char = true;
    }

    // if no word chars, set index after next single non word char
    //if !found_word_char{
    //    if index < text.len_chars() 
    //    && !is_word_char(text.char(index)) 
    //    && !is_whitespace(text.char(index)){
    //        index = next_grapheme_index(index, text);
    //    }
    //}
    if !found_word_char
    && index < text.len_chars()
    && !is_word_char(text.char(index))
    && !is_whitespace(text.char(index)){
        index = next_grapheme_index(index, text);
    }

    if index < text.len_chars(){
        index
    }else{
        text.len_chars()
    }
}

/// Returns the index of the previous word boundary
#[must_use] pub fn previous_word_boundary(current_position: usize, text: &Rope) -> usize{   //should this be Option<usize>?
    // if current_position == 0{return None;}
    
    let mut index = current_position;

    // Skip any trailing whitespace
    while index > 0 && is_whitespace(text.char(previous_grapheme_index(index, text))){
        index = previous_grapheme_index(index, text);
    }

    // Skip to start of word chars, if any
    let mut found_word_char = false;
    while index > 0 && is_word_char(text.char(previous_grapheme_index(index, text))){
        index = previous_grapheme_index(index, text);
        found_word_char = true;
    }

    // if no word chars, set index before next single non word char
    if !found_word_char{    //&& !found_whitespace
        if index > 0
        && !is_word_char(text.char(previous_grapheme_index(index, text))) 
        && !is_whitespace(text.char(previous_grapheme_index(index, text))){
            index = previous_grapheme_index(index, text);
        }
    }

    if index > 0{
        index
    }else{
        0
    }
}

/// Returns true if slice contains only spaces.
#[must_use] pub fn slice_is_all_spaces(slice: RopeSlice) -> bool{
    for char in slice.chars(){
        if char != ' '{
            return false;
        }
    }

    true
}

/// Returns the grapheme distance to next multiple of user defined tab width.
#[must_use] pub fn distance_to_next_multiple_of_tab_width(selection: &Selection, text: &Rope, semantics: CursorSemantics) -> usize{
    let next_tab_distance = offset_from_line_start(selection.cursor(text, semantics), text) % TAB_WIDTH;
    //if offset_from_line_start(selection.cursor(semantics), text) % TAB_WIDTH != 0{
    if next_tab_distance != 0{
        //TAB_WIDTH.saturating_sub(offset_from_line_start(selection.cursor(semantics), text) % TAB_WIDTH)
        TAB_WIDTH.saturating_sub(next_tab_distance)
    }else{
        0
    }
}

/// Returns the offset of cursor position from the start of a line of text.
// TODO: maybe this really does belong in [Selection] in selection.rs?
#[must_use] pub fn offset_from_line_start(point: usize, text: &Rope) -> usize{
    let line_start = text.line_to_char(text.char_to_line(point));
    point.saturating_sub(line_start)
}

// /// Returns the start index of the first matching pattern inside a text if one exists, or None
// // AI gen code. verify behavior and clarify code intent at some later point.
// #[must_use] pub fn naive_search(text: &str, pattern: &str) -> Option<usize> {
//     let text_len = text.len();
//     let pattern_len = pattern.len();
// 
//     if pattern_len == 0 || pattern_len > text_len {
//         return None;
//     }
// 
//     //for i in 0..=text_len - pattern_len {
//     //    if &text[i..i + pattern_len] == pattern {
//     //        return Some(i); // return the index where pattern starts
//     //    }
//     //}
// //
//     //None // if pattern is not found
//     (0..=text_len - pattern_len).find(|&i| &text[i..i + pattern_len] == pattern)    //clippy suggestion
// }
// 
// // AI gen code. verify behavior and clarify code intent at some later point.
// fn build_lps(pattern: &str) -> Vec<usize> {
//     let m = pattern.len();
//     let mut lps = vec![0; m]; // Longest prefix suffix array
//     let mut length = 0; // Length of the previous longest prefix suffix
//     let mut i = 1; // Start from the second character of the pattern
// 
//     // Loop to fill the lps array
//     while i < m {
//         if pattern.as_bytes()[i] == pattern.as_bytes()[length]{
//             length += 1;
//             lps[i] = length;
//             i += 1;
//         }else{
//             if length != 0{
//                 length = lps[length - 1];
//             }else{
//                 lps[i] = 0;
//                 i += 1;
//             }
//         }
//     }
// 
//     lps
// }
// 
// // AI gen code. verify behavior and clarify code intent at some later point.
// #[must_use] pub fn kmp_search(text: &str, pattern: &str) -> Option<usize> {
//     let n = text.len();
//     let m = pattern.len();
// 
//     if m == 0 {
//         return Some(0); // Empty pattern is trivially found at index 0
//     }
// 
//     let lps = build_lps(pattern); // Build the longest prefix suffix array
//     let mut i = 0; // Index for text
//     let mut j = 0; // Index for pattern
// 
//     // Start searching through the text
//     while i < n {
//         if text.as_bytes()[i] == pattern.as_bytes()[j] {
//             i += 1;
//             j += 1;
//         }
// 
//         if j == m {
//             // Pattern found, return starting index
//             return Some(i - j);
//         } else if i < n && text.as_bytes()[i] != pattern.as_bytes()[j] {
//             if j != 0 {
//                 j = lps[j - 1];
//             } else {
//                 i += 1;
//             }
//         }
//     }
// 
//     None // Pattern not found
// }
