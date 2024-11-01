impl Solution {
    pub fn minimum_total_distance(mut robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        let mut factory_flatten: Vec<i32> = Vec::new();
        for f in factory { factory_flatten.extend(vec![f[0]; f[1] as usize]) }
        factory_flatten.sort_unstable();
        robot.sort_unstable();

        let mut dp: Vec<i64> = vec![0; factory_flatten.len() + 1];

        for r in robot.into_iter() {
            let prev_dp = dp.clone();

            dp[0] = i64::MAX / 2;
            for (i, f) in factory_flatten.iter().enumerate() { dp[i+1] = dp[i].min((r - f).abs() as i64 + prev_dp[i]) }
        }

        *dp.last().unwrap()
    }
}
