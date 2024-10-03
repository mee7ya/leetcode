use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let p: i64 = p as i64;
        let nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
        let total: i64 = nums.iter().sum::<i64>() % p; 
        if total == 0 { return 0 }

        let mut map: HashMap<i64, i64> = HashMap::new();
        let mut acc: i64 = 0;
        let mut min_len: i64 = nums.len() as i64;
        for (i, num) in nums.iter().enumerate() {
            acc = (acc + num) % p;
            if acc == total { min_len = min_len.min(i as i64 + 1) }
            if let Some(value) = map.get(&((acc - total + p) % p)) { min_len = min_len.min(i as i64 - *value) }
            map.insert(acc, i as i64);
        }
        if min_len < nums.len() as i64 { min_len as i32 } else { -1 }
    }
}
