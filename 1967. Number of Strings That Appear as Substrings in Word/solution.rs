impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns.into_iter().filter(|x| word.contains(x)).count() as i32
    }
}
