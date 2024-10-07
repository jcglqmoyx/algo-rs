use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut stations = stations;
        stations.push(vec![target, 0]);
        let mut s = start_fuel;
        let mut res = 0;
        let mut heap = BinaryHeap::new();
        for station in &stations {
            while !heap.is_empty() && s < station[0] {
                s += heap.pop().unwrap_or(&0);
                res += 1;
            }
            if s < station[0] {
                return -1;
            }
            heap.push(&station[1]);
        }
        res
    }
}