struct Solution;

impl Solution {
    pub fn losing_player(x: i32, y: i32) -> String {
        use std::cmp::min;
        if (min(x, y / 4) & 1) == 1 {
            String::from("Alice")
        } else {
            String::from("Bob")
        }
    }
}