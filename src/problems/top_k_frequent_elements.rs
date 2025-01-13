use std::cmp::Reverse;

pub struct Solution;

pub fn run() {
    let res = Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
    println!("{res:?}");
}
// is faster :(
impl Solution {
    pub fn _top_k_frequent_sort(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut frequency: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            frequency.entry(num).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut res = frequency.iter().collect::<Vec<_>>();
        res.sort_by(|a, b| b.1.cmp(a.1));
        res.iter().take(k as usize).map(|(key, _)| **key).collect()
    }
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::BinaryHeap;
        use std::collections::HashMap;

        let mut frequency: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            frequency.entry(num).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut heap = BinaryHeap::new();

        frequency.into_iter().for_each(|(key, value)| {
            heap.push((value, key));
        });

        let mut result = Vec::new();
        for _ in 0..k {
            if let Some((_, num)) = heap.pop() {
                result.push(num);
            }
        }

        result
    }
}
