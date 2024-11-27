use std::collections::VecDeque;

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut distances: Vec<i32> = (0..n).collect();
        let mut direction: Vec<Vec<usize>> = (0..n).map(|x| if x != n - 1 { vec![(x+1) as usize] } else { Vec::<usize>::new() }).collect();
        let mut deque: VecDeque<usize> = VecDeque::new();
        let mut result: Vec<i32> = Vec::with_capacity(queries.len());
        for q in queries {
            let from: usize = q[0] as usize;
            let to: usize = q[1] as usize;
            direction[from].push(to);
            deque.push_back(from);
            while !deque.is_empty() {
                let current: usize = deque.pop_front().unwrap();
                
                for next in direction[current].iter() {
                    if distances[current] + 1 < distances[*next] {
                        distances[*next] = distances[current] + 1;
                        deque.push_back(*next);
                    }
                }
            }
            result.push(*distances.last().unwrap());
        }
        result
    }
}
