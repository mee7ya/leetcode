impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let (mut left, mut right): (i32, i32) = (0, *quantities.iter().max().unwrap());
        while left < right {
            let middle = (left + right) / 2;
            let mut current: usize = 0;
            let mut quantity = quantities[current];
            let mut can_distribute = false;
            for i in (0..n) {
                if quantity <= middle {
                    current += 1;
                    if current >= quantities.len() {
                        can_distribute = true;
                        break
                    }
                    quantity = quantities[current];
                } else {
                    quantity -= middle;
                }
            }
            if can_distribute {
                right = middle;
            } else {
                left = middle + 1;
            }
        }
        left
    }
}
