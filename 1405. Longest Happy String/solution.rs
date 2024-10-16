impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut vec = vec![(a, 'a'), (b, 'b'), (c, 'c')];
        let mut most_idx: usize = 2;
        let mut result: Vec<char> = Vec::new();
        vec.sort_unstable_by_key(|k| k.0);
        while vec[0].0 + vec[1].0 + vec[2].0 > 0 {
            if vec[most_idx].0 == 0 { most_idx -= 1 }
            if most_idx > 2 { break }

            if result.len() > 1 {
                if result[result.len() - 1] == result[result.len() - 2] && result[result.len() - 2] == vec[most_idx].1 {
                    let (sep, idx) = if most_idx - 1 <= 2 && vec[most_idx - 1].0 > 0 { (vec[most_idx - 1].1, most_idx - 1) } 
                    else if most_idx - 2 <= 2 && vec[most_idx - 2].0 > 0 { (vec[most_idx - 2].1, most_idx - 2) }
                    else { break };
                    result.push(sep);
                    vec[idx].0 -= 1;
                    continue
                }
            }
            result.push(vec[most_idx].1);
            vec[most_idx].0 -= 1;
            vec.sort_unstable_by_key(|k| k.0);
        }
        result.into_iter().collect::<String>()
    }
}
