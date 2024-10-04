impl Solution {
    pub fn divide_players(mut skill: Vec<i32>) -> i64 {
        skill.sort_unstable();
        let skill_to_find: i32 = skill.first().unwrap() + skill.last().unwrap();
        let mut chemistry: i64 = 0;
        for i in (0..skill.len() / 2) {
            let team: (&i32, &i32) = (&skill[i], &skill[skill.len() - 1 - i]);
            if *team.0 + *team.1 != skill_to_find { return -1 }
            chemistry += (*team.0 * *team.1) as i64;
        }
        chemistry
    }
}
