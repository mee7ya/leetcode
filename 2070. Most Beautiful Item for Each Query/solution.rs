impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        items.sort_unstable();
        let mut max_beauty: i32 = -1;
        let mut items_unique: Vec<Vec<i32>> = Vec::new();

        for item in items {
            max_beauty = max_beauty.max(item[1]);
            match items_unique.last_mut() {
                Some(val) => {
                    if val[0] == item[0] {
                        val[1] = max_beauty;
                    } else {
                        items_unique.push(Vec::from([item[0], max_beauty]));
                    }
                },
                None => items_unique.push(Vec::from([item[0], max_beauty])),
            }
        }

        let mut answers: Vec<i32> = Vec::with_capacity(queries.len());
        for q in queries {
            let pos = match items_unique.binary_search_by_key(&q, |x| x[0]) {
                Ok(pos) => pos,
                Err(pos) => pos - 1,
            };
            answers.push(if pos != usize::MAX { items_unique[pos][1] } else { 0 })
        }
        answers
    }
}
