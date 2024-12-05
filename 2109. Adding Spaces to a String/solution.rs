impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let spaces_len = spaces.len();
        let mut result: String = String::with_capacity(s.len() + spaces.len());
        let mut spaces_idx = 0;
        if spaces[spaces_idx] == 0 {
            result.push(' ');
            spaces_idx += 1;
        }
        for (i, c) in s.chars().enumerate() {
            result.push(c);
            if spaces_idx < spaces.len() && spaces[spaces_idx] as usize == i + 1 {
                result.push(' ');
                spaces_idx += 1;
            }
        }
        result
    }
}
