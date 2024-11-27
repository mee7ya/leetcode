impl Solution {
    pub fn max_matrix_sum(mut matrix: Vec<Vec<i32>>) -> i64 {
        let n: usize = matrix.len();
        let m: usize = matrix[0].len();
        let mut subtract: Vec<i32> = Vec::new();
        let mut min_value: i32 = i32::MAX;
        for i in (0..n) {
            for j in (0..m - 1) {
                if matrix[i][j] < 0 {
                    matrix[i][j] = -matrix[i][j];
                    matrix[i][j+1] = -matrix[i][j+1];
                }
                min_value = min_value.min(matrix[i][j].abs());
            }
            min_value = min_value.min(matrix[i][m-1].abs());
        }
        for i in (0..n-1) {
            if matrix[i][m-1] < 0 {
                matrix[i][m-1] = -matrix[i][m-1];
                matrix[i+1][m-1] = -matrix[i+1][m-1];
            }
        }
        if matrix[n-1][m-1] < 0 {
            matrix[n-1][m-1] = -matrix[n-1][m-1];
        } else {
            min_value = 0;
        }
        matrix.into_iter().flatten().map(|x| x as i64).sum::<i64>() - 2 * (min_value as i64)
    }
}
