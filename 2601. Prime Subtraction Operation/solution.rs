impl Solution {
    pub fn prime_sub_operation(mut nums: Vec<i32>) -> bool {
        let mut primes: Vec<i32> = Vec::new();
        'outer: for i in (2..1000) {
            for p in primes.iter() {
                if i % p == 0 { continue 'outer }
            }
            primes.push(i as i32);
        }
        for i in (0..nums.len()) {
            let slice: &[i32] = match primes.binary_search(&nums[i]) {
                Ok(val) => &primes[..val],
                Err(val) => &primes[..val],
            };
            for p in slice.into_iter().rev() {
                if i > 0 {
                    if nums[i] - p > nums[i-1] {
                        nums[i] = nums[i] - p;
                        break;
                    }
                } else {
                    nums[i] = nums[i] - p;
                    break
                }
            }
        }
        nums.windows(2).all(|x| x[1] > x[0])
    }
}
