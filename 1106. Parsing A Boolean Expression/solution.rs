impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        Self::parse(&expression)
    }

    pub fn parse(expression: &str) -> bool {
        let c = expression.chars().next().unwrap();
        match c {
            'f' => return false,
            't' => return true,
            _ => {
                let subexpression: &str = &expression[2..expression.len() - 1];
                if c == '!' { return !Self::parse(subexpression) }
                let mut parenthesis = 0;
                let mut splits = subexpression.split(|x| match x {
                    '(' => {
                        parenthesis += 1;
                        false
                    },
                    ')' => {
                        parenthesis -= 1;
                        false
                    },
                    ',' => if parenthesis == 0 { true } else { false },
                    _ => false,
                });
                match c {
                    '&' => return splits.all(Self::parse),
                    '|' => return splits.any(Self::parse),
                    _ => panic!("wtf"),
                }
            },
        }
    }
}
