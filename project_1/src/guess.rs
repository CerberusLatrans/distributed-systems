use std::collections::HashMap;

use crate::message::Score;

pub fn filter_words(history: Vec<Score>, mut word_list: Vec<&str>) -> Vec<&str> {
    let last_history = history.last();
    if let Some(last_score) = last_history {
        let word: Vec<char> = last_score.word.chars().collect();
        let marks = last_score.marks.clone();
        let mut contained = Vec::new();
        let mut correct = HashMap::new();
        let mut absent = Vec::new();
        for i in 0..word.len() {
            match marks[i] {
                2 => {correct.insert(i, word[i]);},
                1 => contained.push(word[i]),
                0 => absent.push(word[i]),
                _ => panic!("Unknown mark: {}", marks[i]),
            }
        }
        let correct_letters: Vec<char> = correct.values().cloned().collect();
        absent.retain(|s| !correct_letters.contains(s) && !contained.contains(s));
        word_list.retain(
            |s| -> bool {
                if s==&last_score.word {
                    return false
                }
                let word_chars: Vec<char> = s.chars().collect();
                for c in &absent {
                    if word_chars.contains(c) {
                        return false
                    }
                }
                for c in &contained {
                    if !word_chars.contains(c) {
                        return false
                    }
                }
                for (k,v) in correct.iter() {
                    if word_chars[*k]!= *v {
                        return false
                    }
                }
                true
            }
        );
    }
    word_list
}