impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut start: usize = 0;
        let mut consec: usize = 1;
        let mut result: Vec<i32> = Vec::new();
        for end in (0..nums.len()) {
            let el = nums[end];
            if end > 0 && el - nums[end - 1] == 1 {
                consec += 1;
            } else {
                consec = 1;
            }

            if end - start + 1 > k {
                start += 1;
            }
            
            if end - start + 1 == k {
                if consec >= k {
                    result.push(el);
                } else {
                    result.push(-1);
                }
            }
        }
        result
    }
}
