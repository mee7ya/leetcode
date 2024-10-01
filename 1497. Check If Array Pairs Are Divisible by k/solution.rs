use std::collections::HashMap;

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mod_to_count_map: HashMap<i32, usize> = arr.into_iter().map(|x| (x % k + k) % k).fold(HashMap::<i32, usize>::new(), |mut acc, x| {
            acc.entry(x).and_modify(|count| *count += 1).or_insert(1);
            acc
        });

        for i in 0..k {
            if let Some(first) = mod_to_count_map.get(&i) {
                if i == 0 { if first % 2 != 0 { return false } else { continue } }
                if let Some(second) = mod_to_count_map.get(&(k - i).max(1)) { if first != second { return false } } else { return false }
            }
        }
        true
    }
}
