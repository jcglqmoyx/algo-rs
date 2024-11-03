struct Solution;

use std::cmp::min;
use std::collections::HashMap;

impl Solution {
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let n = price.len();
        let mut special_offer: Vec<&Vec<i32>> = Vec::new();
        let mut f: HashMap<Vec<i32>, i32> = HashMap::new();

        for offer in special.iter() {
            let mut tot = 0;
            for i in 0..n {
                tot += price[i] * offer[i];
            }
            if tot > offer[n] {
                special_offer.push(offer);
            }
        }

        fn dp(price: &Vec<i32>, need: &[i32], special_offer: &Vec<&Vec<i32>>, f: &mut HashMap<Vec<i32>, i32>) -> i32 {
            let n = price.len();
            if let Some(&val) = f.get(need) {
                return val;
            }

            let mut res = 0;
            for i in 0..n {
                res += price[i] * need[i];
            }

            for offer in special_offer.iter() {
                let mut t = vec![];
                for i in 0..n {
                    if offer[i] > need[i] {
                        break;
                    }
                    t.push(need[i] - offer[i]);
                }
                if t.len() == n {
                    res = min(res, dp(price, &t, special_offer, f) + offer[n]);
                }
            }

            f.insert(need.to_vec(), res);
            res
        }

        dp(&price, &needs, &special_offer, &mut f)
    }
}