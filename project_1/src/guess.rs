use crate::message::Score;

pub fn make_guess(history: Vec<Score>, mut word_list: &Vec<&str>) -> String {
    //let last_score = history.last().unwrap();
    //let word: Vec<char> = last_score.word.chars().collect();
    //let marks = last_score.marks;
    //let positional = Vec::new();
    //let corrects = Vec::new();
    //let not_in = Vec::new();
    //for i in 1..6 {
    //    match marks[i] {
    //        0 => not_in.push(word[i]),
    //        1 => positional.push(word[i]),
    //        2 => corrects.push(),
    //    }
    //}
    //word_list[0].to_owned()
    word_list[history.len()].to_owned()
}