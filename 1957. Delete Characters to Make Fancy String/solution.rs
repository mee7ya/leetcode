impl Solution {
    pub fn make_fancy_string(mut s: String) -> String {
        s.as_bytes().chunk_by(|a, b| a == b).flat_map(|x| x.iter().take(2).map(|c| *c as char)).collect::<String>()
    }
}
