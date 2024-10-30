impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; grid[0].len()]; grid.len()];
        let mut stack: Vec<(usize, usize)> = Vec::new();
        for i in (0..dp.len()) { stack.push((i, 0)) }
        
        while !stack.is_empty() {
            let (i, j) = stack.pop().unwrap();
            if j + 1 < dp[0].len() {
                for next_i in [i-1, i, i+1] {
                    if next_i < dp.len() && grid[next_i][j+1] > grid[i][j] {
                        let old = dp[next_i][j+1];
                        dp[next_i][j+1] = dp[next_i][j+1].max(dp[i][j] + 1);
                        if old != dp[next_i][j+1] { stack.push((next_i, j+1)) }
                    }
                }
            }
        }
        
        dp.into_iter().flatten().max().unwrap()
    }
}
