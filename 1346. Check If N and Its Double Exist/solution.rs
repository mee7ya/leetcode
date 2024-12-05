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
use std::collections::HashSet;

impl Solution {
    pub fn check_if_exist(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        let n: usize = arr.len();
        let (first, last): (i32, i32) = (arr[0], arr[n - 1]);
        let mut neg_set: HashSet<i32> = HashSet::new();
        let mut pos_set: HashSet<i32> = HashSet::new();
        let (mut left, mut right): (usize, usize) = (0, n - 1);
        while left < n {
            let mut val: i32 = 2 * arr[left];
            if val < 0 {
                if neg_set.contains(&val) {
                    return true
                }
                neg_set.insert(arr[left]);
            } else {
                if pos_set.contains(&val) {
                    return true
                }
            }
            val = 2 * arr[right];
            if val < 0 {
                if neg_set.contains(&val) {
                    return true
                }
            } else {
                if pos_set.contains(&val) {
                    return true
                }
                pos_set.insert(arr[right]);
            }
            left += 1;
            right -= 1;
        }
        false
    }
}
