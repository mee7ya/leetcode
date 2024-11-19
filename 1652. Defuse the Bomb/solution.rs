impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        code.iter().enumerate().map(|(i, x)| {
            if k == 0 { 0 }
            else if k > 0 {
                (1..k+1).map(|j| code[(i + j as usize) % code.len()]).sum()
            } else {
                (k..0).rev().map(|j| {
                    code[(((i as i32 + j) % code.len() as i32 + code.len() as i32) % code.len() as i32) as usize]
                }).sum()
            }
        }).collect()
    }
}
