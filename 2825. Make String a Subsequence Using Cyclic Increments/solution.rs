impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let n: usize = str1.len();
        let k: usize = str2.len();
        let str1_bytes: Vec<u8> = str1.as_bytes().into_iter().map(|x| x - 'a' as u8).collect();
        let str2_bytes: Vec<u8> = str2.as_bytes().into_iter().map(|x| x - 'a' as u8).collect();
        let (mut str1_iter, mut str2_iter): (usize, usize) = (0, 0);
        while str1_iter < n && str2_iter < k {
            if str1_bytes[str1_iter] == str2_bytes[str2_iter] || (str1_bytes[str1_iter] + 1) % 26 == str2_bytes[str2_iter] {
                str2_iter += 1
            }
            str1_iter += 1
        }
        return str2_iter == k
    }
}
