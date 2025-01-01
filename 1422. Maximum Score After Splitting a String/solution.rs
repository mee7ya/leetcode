impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s: Vec<u8> = s.into_bytes();
        let mut max_score: i32 = -1;
        let mut left_substr: i32 = 0;
        let mut right_substr: i32 = s.iter().filter(|&&x| x == 49_u8).count() as i32;
        for i in (0..s.len() - 1) {
            match s[i] {
                48_u8 => left_substr += 1,
                49_u8 => right_substr -= 1,
                _ => {},
            }
            max_score = max_score.max(left_substr + right_substr);
        }
        max_score
    }
}
