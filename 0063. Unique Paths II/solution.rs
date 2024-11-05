impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if *obstacle_grid.last().unwrap().last().unwrap() == 1 { return 0 }

        let mut dp: Vec<Vec<i32>> = vec![vec![0; obstacle_grid[0].len()]; obstacle_grid.len()];
        *dp.last_mut().unwrap().last_mut().unwrap() = 1;

        for i in (0..dp.len()).rev() {
            for j in (0..dp[i].len()).rev() {
                if obstacle_grid[i][j] == 1 { continue }
                if j + 1 < dp[i].len() { dp[i][j] += dp[i][j+1] }
                if i + 1 < dp.len() { dp[i][j] += dp[i+1][j]}
            }
        }

        dp[0][0]
    }
}
