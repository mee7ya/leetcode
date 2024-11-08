impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut arr: [i32; 24] = [0; 24];
        let mut max_size: i32 = 0;
        for mut num in candidates {
            let mut position: usize = 0;
            while num != 0 {
                if num & 1 == 1 { arr[position] += 1; }
                max_size = max_size.max(arr[position]);
                num >>= 1;
                position += 1;
            }
        }
        max_size
    }
}
