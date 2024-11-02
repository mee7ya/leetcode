impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let split: Vec<&[u8]> = sentence.split_whitespace().map(|x| x.as_bytes()).collect();
        if split.first().unwrap().first().unwrap() != split.last().unwrap().last().unwrap() { return false }
        split.windows(2).all(|x| x.first().unwrap().last().unwrap() == x.last().unwrap().first().unwrap())
    }
}
