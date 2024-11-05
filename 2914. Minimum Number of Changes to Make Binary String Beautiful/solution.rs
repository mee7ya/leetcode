impl Solution {
    pub fn min_changes(s: String) -> i32 {
        s.as_bytes().chunks(2).fold(0, |state: i32, x| if x[0] != x[1] { state + 1 } else { state } )
    }
}
