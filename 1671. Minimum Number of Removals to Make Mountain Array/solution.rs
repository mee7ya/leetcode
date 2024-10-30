impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let mut lis: Vec<i32> = Vec::new();
        let mut max_lis: Vec<usize> = Vec::new();

        for num in nums.iter() {
            if lis.is_empty() || num > lis.last().unwrap() { lis.push(*num); }
            else if let Err(replace) = lis.binary_search(num) { lis[replace] = *num; }
            max_lis.push(lis.len());
        }

        let mut lds: Vec<i32> = Vec::new();
        let mut max_lds: Vec<usize> = Vec::new();

        for num in nums.iter().rev() {
            if lds.is_empty() || num > lds.last().unwrap() { lds.push(*num); }
            else if let Err(replace) = lds.binary_search(num) { lds[replace] = *num; }
            max_lds.push(lds.len());
        }
        max_lds.reverse();

        let mut max_mountain: usize = 0;
        for (lis_val, lds_val) in max_lis.into_iter().zip(max_lds.into_iter()) {
            if lis_val > 1 && lds_val > 1 { max_mountain = max_mountain.max(lis_val + lds_val - 1) }
        }
        (nums.len() - max_mountain) as i32
    }
}
