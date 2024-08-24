struct Solution;

impl Solution {
    fn change(s: &mut Vec<char>) {
        let mut i = 0;
        let mut j = s.len() - 1;
        while i < j {
            s[j] = s[i];
            i += 1;
            j -= 1;
        }
    }

    pub fn nearest_palindromic(n: String) -> String {
        let num = match n.parse::<i64>() {
            Ok(num) => num,
            Err(_) => return "0".to_string(),
        };

        if num < 10 {
            return (num - 1).to_string();
        } else if num < 100 {
            let dest = [9, 11, 22, 33, 44, 55, 66, 77, 88, 99, 101];
            let mut diff = i64::MAX;
            let mut res = 0;
            for &x in &dest {
                let d = (num - x).abs();
                if d != 0 && d < diff {
                    diff = d;
                    res = x;
                }
            }
            return res.to_string();
        }

        let mut s: Vec<char> = n.chars().collect();
        let n_len = s.len();
        Self::change(&mut s);
        let mut v = vec![match s.iter().collect::<String>().parse::<i64>() {
            Ok(v) => v,
            Err(_) => return "0".to_string(),
        }];

        if n_len % 2 != 0 {
            v.push(num + 10_i64.pow((n_len / 2) as u32));
            v.push(num - 10_i64.pow((n_len / 2) as u32));
        } else {
            v.push(num + 10_i64.pow((n_len / 2) as u32));
            v.push(num - 10_i64.pow((n_len / 2) as u32));
            v.push(num + 10_i64.pow((n_len / 2 - 1) as u32));
            v.push(num - 10_i64.pow((n_len / 2 - 1) as u32));
        }

        let mut diff = i64::MAX;
        let mut res = 0;

        for x in v {
            let mut t: Vec<char> = x.to_string().chars().collect();
            Self::change(&mut t);
            let nx = match t.iter().collect::<String>().parse::<i64>() {
                Ok(nx) => nx,
                Err(_) => continue,
            };
            let d = (num - nx).abs();

            if d != 0 && (d < diff || (d == diff && nx < res)) {
                diff = d;
                res = nx;
            }
        }

        res.to_string()
    }
}