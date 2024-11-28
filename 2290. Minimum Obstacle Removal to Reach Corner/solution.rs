use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let mut heap: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
        let mut dp: Vec<Vec<i32>> = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
        dp[0][0] = 0;
        heap.push((0_i32, 0_usize, 0_usize));

        while !heap.is_empty() {
            let (cost, i, j): (i32, usize, usize) = heap.pop().unwrap();
            let cost = -cost;
            for (new_i, new_j) in [
                (i - 1, j),
                (i, j + 1),
                (i + 1, j),
                (i, j - 1),
            ] {
                if new_i < grid.len() && new_j < grid[0].len() {
                    if grid[new_i][new_j] == 0 && dp[i][j] < dp[new_i][new_j] {
                        dp[new_i][new_j] = dp[i][j];
                        heap.push((-dp[new_i][new_j], new_i, new_j));
                    } else if grid[new_i][new_j] == 1 && dp[i][j] + 1 < dp[new_i][new_j] {
                        dp[new_i][new_j] = dp[i][j] + 1;
                        heap.push((-dp[new_i][new_j], new_i, new_j));
                    }
                }
                if new_i == grid.len() && new_j == grid.len() - 1 {
                    break;
                }
            }
        }
        *dp.last().unwrap().last().unwrap()
    }
}
