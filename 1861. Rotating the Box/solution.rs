impl Solution {
    pub fn rotate_the_box(mut box_: Vec<Vec<char>>) -> Vec<Vec<char>> {
        for line in (0..box_.len()) {
            let mut right: usize = box_[0].len() - 1;
            while right < box_[0].len() && box_[line][right] != '.' {
                right -= 1;
            }

            let mut left: usize = right - 1;
            while left < box_[0].len() {
                if box_[line][left] == '#' {
                    box_[line][left] = '.';
                    box_[line][right] = '#';
                    while right < box_[0].len() && box_[line][right] != '.' {
                        right -= 1;
                    }
                    left -= 1;
                } else if box_[line][left] == '*' {
                    right = if right - 1 < box_[0].len() { right - 1 } else { 0 };
                    while right < box_[0].len() && box_[line][right] != '.' {
                        right -= 1;
                    }
                    left = right - 1;
                } else {
                    left -= 1;
                }
            }
        }
        (0..box_[0].len()).map(|i| (0..box_.len()).map(|j| box_[box_.len() - 1 - j][i]).collect()).collect()
    }
}
