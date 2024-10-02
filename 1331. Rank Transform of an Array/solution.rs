use std::collections::HashMap;

impl Solution {
    pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
        let mut sorted = arr.clone();
        let mut rank = HashMap::<i32, i32>::new();
        sorted.sort();
        sorted.dedup();
        for (i, value) in sorted.into_iter().enumerate() {
            rank.insert(value, (i + 1) as i32);
        }
        arr.into_iter().map(|x| rank[&x]).collect()
    }
}
