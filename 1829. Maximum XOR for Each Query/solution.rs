impl Solution {
    pub fn get_maximum_xor(mut nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut total_xor: i32 = nums.iter().cloned().reduce(|a, b| a ^ b).unwrap();
        let mut result: Vec<i32> = Vec::with_capacity(nums.len());
        let mask: i32 = (1 << maximum_bit) - 1;
        for i in (0..nums.len()) {
            result.push(total_xor ^ mask);
            total_xor ^= nums.pop().unwrap();
        }
        result
    }
}
