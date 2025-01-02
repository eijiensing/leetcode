pub struct Solution;

pub fn run() {
    let res = Solution::two_sum(vec![2, 7, 11, 15], 9);
    println!("{res:?}");
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut m: HashMap<i32, i32> = HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            match m.get(&(target - *v)) {
                Some(&i2) => return vec![i as i32, i2],
                None => m.insert(*v, i as i32),
            };
        }
        vec![]
    }
}
