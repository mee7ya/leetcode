use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][1] > 1 && grid[1][0] > 1 {
            return -1
        }

        let mut heap: BinaryHeap<(Reverse<usize>, usize, usize)> = BinaryHeap::from([(Reverse(0), 0, 0)]);
        let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
        visited[0][0] = true;
        let mut lowest: i32 = i32::MAX;
        while !heap.is_empty() {
            let (time, i, j): (Reverse<usize>, usize, usize) = heap.pop().unwrap();
            let time: usize = time.0;
            for (new_i, new_j) in [
                (i - 1, j),
                (i, j + 1),
                (i + 1, j),
                (i, j - 1),
            ] {
                if new_i < grid.len() && new_j < grid[0].len() && !visited[new_i][new_j] {
                    if grid[new_i][new_j] as usize > time + 1 {
                        let mut offset: usize = (grid[new_i][new_j] as usize - (time + 1)) % 2;
                        if new_i == grid.len() - 1 && new_j == grid[0].len() - 1 {
                            return grid[new_i][new_j] + offset as i32
                        } else {
                            heap.push((Reverse(grid[new_i][new_j] as usize + offset), new_i, new_j));
                        }
                    } else {
                        if new_i == grid.len() - 1 && new_j == grid[0].len() - 1 {
                            return (time + 1) as i32
                        } else {
                            heap.push((Reverse(time + 1), new_i, new_j));                        
                        }
                    }
                    visited[new_i][new_j] = true;
                }
            }
        }
        return -1
    }
}