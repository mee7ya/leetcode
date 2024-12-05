use std::collections::HashMap;

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut adj: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut degrees: HashMap<i32, (usize, usize)> = HashMap::new();

        for pair in &pairs {
            adj.entry(pair[0]).and_modify(|x| x.push(pair[1])).or_insert(vec![pair[1]]);
            degrees.entry(pair[0]).and_modify(|(x, y)| *x += 1).or_insert((1, 0));
            degrees.entry(pair[1]).and_modify(|(x, y)| *y += 1).or_insert((0, 1));
        }

        let start: i32 = degrees.iter().find(|(&key, &value)| value.0 > value.1).and_then(|(&key, &value)| Some(key)).unwrap_or(pairs[0][0]);
        let mut stack: Vec<i32> = vec![start];

        let mut path: Vec<i32> = vec![];
        while let Some(node) = stack.last() {
            if let Some(next) = adj.get_mut(node).unwrap_or(&mut vec![]).pop() {
                stack.push(next);
            } else {
                path.push(*node);
                stack.pop();
            }
        }

        let mut result: Vec<Vec<i32>> = vec![];
        for window in path.into_iter().rev().collect::<Vec<i32>>().windows(2) {
            result.push(window.to_vec());
        }

        result
    }
}
