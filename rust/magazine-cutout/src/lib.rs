// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_words = hash_words(magazine).unwrap();
    let mut note_words = hash_words(note).unwrap();

    for word in note {
        if let Some(magazine_count) = magazine_words.get(*word) {
            if !(note_words[*word] <= *magazine_count) {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn hash_words(words: &[&str]) -> Option<HashMap<String, usize>> {
    let mut result = HashMap::new();
    for word in words {
        *result.entry(word.to_string()).or_default() += 1;
    }
    Some(result)
}
