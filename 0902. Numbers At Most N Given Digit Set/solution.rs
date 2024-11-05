impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let n: String = n.to_string();
        let n_digits: Vec<char> = n.chars().collect();
        let digits: Vec<char> = digits.into_iter().map(|x| x.chars().next().unwrap()).collect();

        let mut smaller_sum: i32 = 0;
        for i in (1..n_digits.len()) {
            smaller_sum += digits.len().pow(i as u32) as i32;
        }

        let mut dp: Vec<i32> = vec![0; n_digits.len() + 1];
        *dp.last_mut().unwrap() = 1;

        for i in (0..n_digits.len()).rev() {
            for d in digits.iter() {
                if *d < n_digits[i] {
                    dp[i] += digits.len().pow((n_digits.len() - i - 1) as u32) as i32;
                } else if *d == n_digits[i] {
                    dp[i] += dp[i+1];
                }
            }
        }
        dp[0] + smaller_sum
    }
}
