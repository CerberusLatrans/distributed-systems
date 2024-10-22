use std::collections::HashMap;

use crate::message::Score;

pub fn make_guess(history: Vec<Score>, mut word_list: &Vec<&str>) -> String {
    let last_score = history.last().unwrap();
    let word: Vec<char> = last_score.word.chars().collect();
    let marks = last_score.marks;
    let mut contained = Vec::new();
    let mut correct = HashMap::new();
    let mut absent = Vec::new();
    for i in 1..6 {
        match marks[i] {
            0 => absent.push(word[i]),
            1 => contained.push(word[i]),
            2 => {correct.insert(i, word[i]);},
        }
    }
    let is_valid = |s: &str| -> bool {
        let word: Vec<char> = s.chars().collect();
        for c in absent {
            if word.contains(&c) {
                return false
            }
        }
        for c in contained {
            if !word.contains(&c) {
                return false
            }
        }
        for (k,v) in correct.iter() {
            if word[*k]!= *v {
                return false
            }
        }
        true
    };
    word_list = word_list.into_iter().filter(is_valid).collect();
    word_list[0].to_owned()
    //word_list[history.len()].to_owned()
}