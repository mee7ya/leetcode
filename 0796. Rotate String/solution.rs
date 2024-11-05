impl Solution {
    pub fn rotate_string(mut s: String, goal: String) -> bool {
        if s.len() != goal.len() { return false }
        for i in (0..s.len()) {
            if &s[..i] == &goal[goal.len() - i..] && &s[i..] == &goal[..s.len() - i] { return true }
        }
        false
    }
}
