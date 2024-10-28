impl Solution {
    pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
        for i in (1..matrix.len()) {
            for j in (1..matrix[0].len()) {
                if matrix[i][j] == 1 { matrix[i][j] = matrix[i][j-1].min(matrix[i-1][j-1]).min(matrix[i-1][j]) + 1 }
            }
        }
        matrix.into_iter().flatten().sum()
    }
}
