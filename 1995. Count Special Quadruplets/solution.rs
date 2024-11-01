use std::collections::HashMap;

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<usize, Vec<i32>> = HashMap::new();
        let mut total: i32 = 0;
        
        for i in (0..nums.len() - 2) {
            for j in (0..i) {
                if let Some(current) = map.get_mut(&i) {
                    current.push(nums[i] + nums[j]);
                } else {
                    if let Some(prev) = map.get(&(i-1)) {
                        let mut temp = prev.clone();
                        temp.push(nums[i] + nums[j]);
                        map.insert(i, temp);
                    } else {
                        map.insert(i, vec![nums[i] + nums[j]]);
                    }
                }
            }
        }

        for i in (2..nums.len()) {
            for j in ((i+1)..nums.len()) {
                if let Some(current) = map.get(&(i - 1)) {
                    let found = current.iter().filter(|&x| *x == nums[j] - nums[i]).count() as i32;
                    total += found;
                }
            }
        }

        total
    }
}
