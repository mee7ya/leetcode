impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if n == 1 { return '0' }
        let length = 1 << n;
        if k == length / 2 {
            return '1'
        }
        if k < length / 2 {
            return Self::find_kth_bit(n - 1, k)
        }
        if Self::find_kth_bit(n - 1, length - k) == '0' { '1' } else { '0' }
    }
}
