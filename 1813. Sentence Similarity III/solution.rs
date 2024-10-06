use std::collections::VecDeque;
use std::mem::swap;

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let (mut smallest, mut largest): (VecDeque<&str>, VecDeque<&str>) = (
                sentence1.split(' ').collect(), 
                sentence2.split(' ').collect()
            );
        if smallest.len() > largest.len() { swap(&mut smallest, &mut largest) }

        while !smallest.is_empty() {
            if smallest.front().unwrap() == largest.front().unwrap() {
                smallest.pop_front();
                largest.pop_front();
            } else { break }
        }

        while !smallest.is_empty() {
            if smallest.back().unwrap() == largest.back().unwrap() {
                smallest.pop_back();
                largest.pop_back();
            } else { break }
        }

        smallest.is_empty()
    }
}
