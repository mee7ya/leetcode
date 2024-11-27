use std::collections::HashMap;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut map: HashMap<Vec<i32>, i32> = HashMap::new();
        for mut v in matrix {
            if v[0] == 0 {
                v = v.into_iter().map(|x| x ^ 1).collect();
            }
            map.entry(v).and_modify(|x| *x += 1).or_insert(1);
        }
        map.into_values().max().unwrap()
    }
}
