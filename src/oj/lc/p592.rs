struct Solution;

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let mut a = 0;
        let mut b = 1;
        let mut expression = expression;
        if expression.as_bytes()[0] != '-' as u8 {
            expression = String::from('+') + &expression;
        }
        let s = expression.as_bytes();
        let n = s.len();
        let calc = |a, b, c, d, op| {
            let x = if op == '+' as u8 { a * d + b * c } else { a * d - b * c };
            let y = b * d;
            (x, y)
        };
        let mut i = 0;
        while i < n {
            let op = s[i];
            i += 1;
            let mut c = 0;
            let mut d = 0;
            while s[i] != '/' as u8 {
                c = c * 10 + (s[i] as i32) - ('0' as i32);
                i += 1;
            }
            i += 1;
            while i < n && s[i].is_ascii_digit() {
                d = d * 10 + (s[i] as i32) - ('0' as i32);
                i += 1;
            }
            let t = calc(a, b, c, d, op);
            a = t.0;
            b = t.1;
        }
        if a == 0 {
            b = 1;
        }

        fn gcd(a: i32, b: i32) -> i32 {
            if a % b == 0 {
                return b;
            }
            gcd(b, a % b)
        }
        let g = gcd(a, b);
        a /= g;
        b /= g;

        if a * b < 0 && b < 0 {
            a = -a;
            b = -b;
        }

        format!("{}/{}", a, b)
    }
}