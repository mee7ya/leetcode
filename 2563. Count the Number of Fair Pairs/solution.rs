impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();
        let mut total: i64 = 0;
        for (i, num) in nums.iter().enumerate() {
            let left = lower - num;
            let right = upper - num;

            let left_bound = nums.partition_point(|&x| x < left).max(i+1);
            let right_bound = nums.partition_point(|&x| x <= right);
            if left_bound < right_bound {
                total += (right_bound - left_bound) as i64;
            }
        }
        total
    }
}
