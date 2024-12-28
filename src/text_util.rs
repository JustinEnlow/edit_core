use ropey::{Rope, RopeSlice};
use unicode_segmentation::UnicodeSegmentation;
use crate::document::TAB_WIDTH;
use crate::selection::{CursorSemantics, Selection};



/// Returns the count of visible graphemes in a line of text.
/// # Example
/// ```
/// # use ropey::Rope;
/// # use edit_core::text_util;
/// 
/// let text = Rope::from("idk\n");
/// assert!(text_util::line_width(text.slice(..), false) == 3);
/// assert!(text_util::line_width(text.slice(..), true) == 4);
/// 
/// //let text = Rope::from("idk\n\n");
/// //assert!(text_util::line_width(text.slice(..), false) == 3);
/// //assert!(text_util::line_width(text.slice(..), true) == 4);
/// ```
// TODO: handle non standard width chars such as '\t'
pub fn line_width(line: RopeSlice, include_newline: bool) -> usize{
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
/// # Example
/// ```
/// # use ropey::Rope;
/// # use edit_core::text_util;
/// 
/// let text = Rope::from("   idk\n");
/// assert!(text_util::first_non_whitespace_character_offset(text.slice(..)) == 3);
/// 
/// let text = Rope::from("");
/// assert!(text_util::first_non_whitespace_character_offset(text.slice(..)) == 0);
/// 
/// let text = Rope::from("   ");
/// assert!(text_util::first_non_whitespace_character_offset(text.slice(..)) == 0);
/// ```
pub fn first_non_whitespace_character_offset(line: RopeSlice) -> usize{
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

/// Returns true if slice contains only spaces.
/// #Example
/// ```
/// # use edit_core::text_util;
/// # use ropey::RopeSlice;
/// 
/// let text = RopeSlice::from("    ");
/// assert!(text_util::slice_is_all_spaces(text));
/// 
/// let text = RopeSlice::from(" idk ");
/// assert!(!text_util::slice_is_all_spaces(text));
/// ```
pub fn slice_is_all_spaces(slice: RopeSlice) -> bool{
    for char in slice.chars(){
        if char != ' '{
            return false;
        }
    }

    true
}

/// Returns the grapheme distance to next multiple of user defined tab width.
/// # Example
/// ```
/// # use ropey::Rope;
/// # use edit_core::document::TAB_WIDTH;
/// # use edit_core::selection::{Selection, CursorSemantics};
/// # use edit_core::text_util;
/// 
/// let mut tab = String::new();
/// for _ in 0..TAB_WIDTH{
///     tab.push(' ');
/// }
/// let text = Rope::from(format!("{}idk\n", tab));
/// let selection = Selection::new(1, 1);
/// let distance = text_util::distance_to_next_multiple_of_tab_width(selection, &text, CursorSemantics::Bar);
/// assert!(distance == 3);
/// ```
pub fn distance_to_next_multiple_of_tab_width(selection: Selection, text: &Rope, semantics: CursorSemantics) -> usize{
    let next_tab_distance = offset_from_line_start(selection.cursor(semantics), text) % TAB_WIDTH;
    //if offset_from_line_start(selection.cursor(semantics), text) % TAB_WIDTH != 0{
    if next_tab_distance != 0{
        //TAB_WIDTH.saturating_sub(offset_from_line_start(selection.cursor(semantics), text) % TAB_WIDTH)
        TAB_WIDTH.saturating_sub(next_tab_distance)
    }else{
        0
    }
}

/// Returns the offset of cursor position from the start of a line of text.
/// # Example
/// ```
/// # use ropey::Rope;
/// # use edit_core::selection::Selection;
/// # use edit_core::text_util;
/// 
/// let text = Rope::from("idk\nsome\nshit\n");
/// let selection = Selection::new(2, 2);
/// assert!(text_util::offset_from_line_start(selection.head(), &text) == 2);
/// ```
// TODO: maybe this really does belong in [Selection] in selection.rs?
pub fn offset_from_line_start(point: usize, text: &Rope) -> usize{
    let line_start = text.line_to_char(text.char_to_line(point));
    point.saturating_sub(line_start)
}

/// Returns the start index of the first matching pattern inside a text if one exists, or None
pub fn naive_search(text: &str, pattern: &str) -> Option<usize> {
    let text_len = text.len();
    let pattern_len = pattern.len();

    if pattern_len == 0 || pattern_len > text_len {
        return None;
    }

    for i in 0..=text_len - pattern_len {
        if &text[i..i + pattern_len] == pattern {
            return Some(i); // return the index where pattern starts
        }
    }

    None // if pattern is not found
}

fn build_lps(pattern: &str) -> Vec<usize> {
    let m = pattern.len();
    let mut lps = vec![0; m]; // Longest prefix suffix array
    let mut length = 0; // Length of the previous longest prefix suffix
    let mut i = 1; // Start from the second character of the pattern

    // Loop to fill the lps array
    while i < m {
        if pattern.as_bytes()[i] == pattern.as_bytes()[length] {
            length += 1;
            lps[i] = length;
            i += 1;
        } else {
            if length != 0 {
                length = lps[length - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }

    lps
}

pub fn kmp_search(text: &str, pattern: &str) -> Option<usize> {
    let n = text.len();
    let m = pattern.len();

    if m == 0 {
        return Some(0); // Empty pattern is trivially found at index 0
    }

    let lps = build_lps(pattern); // Build the longest prefix suffix array
    let mut i = 0; // Index for text
    let mut j = 0; // Index for pattern

    // Start searching through the text
    while i < n {
        if text.as_bytes()[i] == pattern.as_bytes()[j] {
            i += 1;
            j += 1;
        }

        if j == m {
            // Pattern found, return starting index
            return Some(i - j);
        } else if i < n && text.as_bytes()[i] != pattern.as_bytes()[j] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }

    None // Pattern not found
}
