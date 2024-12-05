impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let mut start: Vec<u8> = start.as_bytes().into_iter().map(|&x| x).collect();
        let target: Vec<u8> = target.as_bytes().into_iter().map(|&x| x).collect();
        let l: u8 = 'L' as u8;
        let r: u8 = 'R' as u8;

        for i in (0..target.len()) {
            if target[i] == l {
                let mut found = false;
                for j in (i..start.len()) {
                    if start[j] == r {
                        return false
                    } 
                    else if start[j] == l {
                        found = true;
                        start.swap(i, j);
                        break;
                    }
                }
                if !found {
                    return false
                }
            }
        }

        for i in (0..target.len()).rev() {
            if target[i] == r {
                let mut found = false;
                for j in (0..=i).rev() {
                    if start[j] == l {
                        return false
                    } 
                    else if start[j] == r {
                        found = true;
                        start.swap(i, j);
                        break
                    }
                }
                if !found {
                    return false
                }
            }
        }
        
        for i in (0..start.len()) {
            if start[i] != target[i] {
                return false
            }
        }
        true
    }
}
