impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut n: i64 = (n - 1) as i64;
        let mut x: i64 = x as i64;
        let mut current_pos: usize = 0;
        while n != 0 {
            let bit = n & 1;
            while (x >> current_pos) & 1 != 0 {
                current_pos += 1;
            }
            x |= bit << current_pos;
            current_pos += 1;
            n >>= 1;
        }
        x
    }
}
