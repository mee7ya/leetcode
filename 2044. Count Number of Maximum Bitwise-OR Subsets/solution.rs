impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        fn backtracking(i: usize, current: i32, max: i32, nums: &Vec<i32>) -> i32 {
            if i == nums.len() { return if current == max { 1 } else { 0 } }
            backtracking(i + 1, current, max, nums) + backtracking(i + 1, current | nums[i], max, nums)
        }

        let max: i32 = nums.iter().fold(0, |acc, x| acc | x);
        backtracking(0, 0, max, &nums)
    }
}
