impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut losers: Vec<bool> = vec![false; n as usize];
        for edge in edges {
            losers[edge[1] as usize] = true;
        }
        let mut winner: i32 = -1;
        for (idx, loser) in losers.into_iter().enumerate() {
            if !loser {
                if winner == -1 {
                    winner = idx as i32;
                } else {
                    winner = -1;
                    break;
                }
            }
        }
        winner
    }
}
