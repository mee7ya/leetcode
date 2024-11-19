use std::collections::HashMap;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut start: usize = 0;
        let mut max_subarray: i64 = 0;
        let mut current: i64 = 0;
        for (end, value) in nums.iter().enumerate() {
            current += *value as i64;
            map.entry(*value).and_modify(|x| *x += 1).or_insert(1);
            
            if end - start + 1 > k {
                current -= nums[start] as i64;
                let mut old = map.get_mut(&nums[start]).unwrap();
                *old -= 1;
                if *old == 0 {
                    map.remove(&nums[start]);
                }
                start += 1
            }

            if end - start + 1 == k && map.len() == k {
                max_subarray = max_subarray.max(current);
            }
        }
        max_subarray
    }
}
