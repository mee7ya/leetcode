impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        fn backtracking(mut current: String, s: &str, original: &String, mut buffer: Vec<String>, store: bool) -> i32 {
            if store {
                if !buffer.contains(&current) { 
                    buffer.push(current);
                    if let Some(c) = s.chars().next() { current = c.to_string(); }
                    else { return buffer.len() as i32 }
                }
                else { return 1 }
            } else {
                if let Some(c) = s.chars().next() { current.push(c); }
                else { return buffer.len() as i32 }
            }
            backtracking(current.clone(), &s[1..s.len()], original, buffer.clone(), true).max(
                backtracking(current, &s[1..s.len()], original, buffer, false)
            )
        }

        backtracking(s.chars().next().unwrap().to_string(), &s[1..s.len()], &s, Vec::<String>::new(), true).max(
            backtracking(s.chars().next().unwrap().to_string(), &s[1..s.len()], &s, Vec::<String>::new(), false)
        )
    }
}
