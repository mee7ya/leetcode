use std::collections::HashSet;

impl Solution {
    pub fn longest_square_streak(mut nums: Vec<i32>) -> i32 {
        let mut set: HashSet<i32> = nums.into_iter().collect();
        let mut max_streak: i32 = -1;
        let max_value_without_overflow: i32 = 46340;
        for num in set.iter() {
            let mut streak: i32 = 1;
            let mut find: i32 = if *num <= max_value_without_overflow { num.pow(2) } else { continue };
            while set.contains(&find) {
                streak += 1;
                find = if find <= max_value_without_overflow { find.pow(2) } else { break };
            }
            if streak > 1 && streak > max_streak {
                max_streak = streak;
            }
        }
        max_streak
    }
}
