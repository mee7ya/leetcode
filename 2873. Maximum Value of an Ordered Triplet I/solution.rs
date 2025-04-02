impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut result: String = String::new();
        let mut count: u8 = 1;
        let mut current_c: char = word.chars().next().unwrap();
        for c in word.chars().skip(1) {
            if current_c == c && count < 9 { count += 1; } 
            else {
                result.push(char::from_digit(count as u32, 10).unwrap());
                result.push(current_c);
                current_c = c;
                count = 1;
            }
        }
        result.push(char::from_digit(count as u32, 10).unwrap());
        result.push(current_c);
        result
    }
}
