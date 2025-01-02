pub struct Solution;

pub fn run() {
    let res = Solution::contains_duplicate(vec![1, 2, 3, 1]);
    println!("{res}");
    let res = Solution::contains_duplicate(vec![1, 2, 3, 4]);
    println!("{res}");
}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        for num in nums {
            if !set.insert(num) {
                return true;
            }
        }
        false
    }
}
