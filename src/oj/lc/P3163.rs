struct Solution;

use std::cmp::min;

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut result = String::new();
        let mut i = 0;
        let s = word.as_bytes();
        while i < s.len() {
            let mut j = i + 1;
            while j < min(i + 9, s.len()) && s[j] == s[i] {
                j += 1;
            }
            let len = j - i;
            result.push_str(len.to_string().as_str());
            result.push(s[i] as char);
            i = j - 1;
            i += 1;
        }
        result
    }
}