struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        use std::collections::HashMap;
        let s = s.as_bytes();
        let mut map = HashMap::new();
        map.insert(')', '(');
        map.insert(']', '[');
        map.insert('}', '{');
        let mut stk = vec![];
        for &c in s {
            if let Some(mat) = map.get(&(c as char)) {
                if stk.is_empty() || stk.last().unwrap() != mat {
                    return false;
                }
                stk.pop();
            } else {
                stk.push(c as char);
            }
        }
        stk.is_empty()
    }
}