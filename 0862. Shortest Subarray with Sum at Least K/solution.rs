use std::collections::VecDeque;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let mut deque: VecDeque<usize> = VecDeque::new();
        let mut prefix_sum: Vec<i64> = vec![0; nums.len() + 1];
        for i in (1..prefix_sum.len()) {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1] as i64;
        }

        let mut min_len: usize = usize::MAX;
        for i in (0..prefix_sum.len()) {
            deque.push_back(i);
            while !deque.is_empty() && prefix_sum[i] - prefix_sum[*deque.front().unwrap()] >= k {
                min_len = min_len.min(i - deque.pop_front().unwrap());
            }
            while !deque.is_empty() && prefix_sum[i] <= prefix_sum[*deque.back().unwrap()] {
                deque.pop_back();
            }
            deque.push_back(i);
        }
        if min_len != usize::MAX { min_len as i32 } else { -1 }
    }
}
