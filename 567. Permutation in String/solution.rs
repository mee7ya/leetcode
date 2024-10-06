impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let n: usize = s1.len();
        let mut to_find: Vec<i16> = vec![0; 26];
        for byte in s1.into_bytes() { to_find[(byte - 'a' as u8) as usize] += 1 }
        's2_loop: for sub_bytes in s2.into_bytes().windows(n) {
            let mut freq: Vec<i16> = to_find.clone();
            for byte in sub_bytes {
                let i: usize = (byte - 'a' as u8) as usize;
                freq[i] -= 1;
                if freq[i] < 0 { continue 's2_loop }
            }
            return true
        }
        false
    }
}
