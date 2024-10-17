impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let n = num.ilog10() + 1;
        let mut max = -1;
        let mut split: Vec<i32> = Vec::with_capacity(n as usize);
        for i in (0..n).rev() {
            let digit = num / 10_i32.pow(i) % 10;
            split.push(digit);
        }
        for i in (0..split.len()) {
            for j in (0..i) {
                if split[i] > split[j] {
                    split.swap(i, j);
                    max = max.max(split.iter().enumerate().fold(0, |acc, (i, x)| acc + x * 10_i32.pow(n - i as u32 - 1)));
                    split.swap(i, j);
                }
            }
        }
        if max != -1 { max } else { num }
    }
}
