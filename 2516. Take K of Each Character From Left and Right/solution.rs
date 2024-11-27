impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let chars: Vec<u8> = s.into_bytes();
        let mut total: Vec<i32> = vec![0; 3];
        let mut current: Vec<i32> = vec![0; 3];
        for c in chars.iter() {
            total[(*c - 'a' as u8) as usize] += 1;
        }
        if total.iter().any(|x| *x < k) {
            return -1
        }
        let mut start: usize = 0;
        let mut max_subarr: usize = 0;
        for end in (0..chars.len()) {
            current[(chars[end] - 'a' as u8) as usize] += 1;
            while start <= end 
            && (total[0] - current[0] < k
            || total[1] - current[1] < k
            || total[2] - current[2] < k) {
                current[(chars[start] - 'a' as u8) as usize] -= 1;
                start += 1;
            }
            max_subarr = max_subarr.max(end - start + 1);
        }
        (chars.len() - max_subarr) as i32
    }
}
