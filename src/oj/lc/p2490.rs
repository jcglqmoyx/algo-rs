struct Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let s = sentence.as_bytes();
        if s[0] != s[s.len() - 1] {
            return false;
        }
        for i in 0..s.len() {
            if s[i] == ' ' as u8 {
                if s[i - 1] != s[i + 1] {
                    return false;
                }
            }
        }
        true
    }
}