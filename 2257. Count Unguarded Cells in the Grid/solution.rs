use std::collections::HashMap;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let m: usize = m as usize;
        let n: usize = n as usize;
        
        let mut grid: Vec<Vec<char>> = vec![vec!['o'; n]; m];

        for wall in walls {
            grid[wall[0] as usize][wall[1] as usize] = 'w';
        }

        for guard in guards {
            grid[guard[0] as usize][guard[1] as usize] = 'g';
        }

        for row in (0..m) {
            for col in (1..n) {
                if grid[row][col] == 'w' { continue };
                grid[row][col] = if grid[row][col] == 'o' && (grid[row][col - 1] == 'g' || grid[row][col - 1] == 'r') { 'r' } else { grid[row][col] };
            }
            for col in (0..n-1).rev() {
                if grid[row][col] == 'w' { continue };
                grid[row][col] = if grid[row][col] == 'o' && (grid[row][col + 1] == 'g' || grid[row][col + 1] == 'r') { 'r' } else { grid[row][col] };
            }
        }

        for col in (0..n) {
            for row in (1..m) {
                if grid[row][col] == 'w' { continue };
                grid[row][col] = if (grid[row][col] == 'o' || grid[row][col] == 'r') && (grid[row - 1][col] == 'g' || grid[row - 1][col] == 'c') { 'c' } else { grid[row][col] };

            }
            for row in (0..m-1).rev() {
                if grid[row][col] == 'w' { continue };
                grid[row][col] = if (grid[row][col] == 'o' || grid[row][col] == 'r') && (grid[row + 1][col] == 'g' || grid[row + 1][col] == 'c') { 'c' } else { grid[row][col] };
            }
        }
        
        grid.into_iter().flatten().fold(0_i32, |state, x| if x == 'o' { state + 1 } else { state })
    }
}
