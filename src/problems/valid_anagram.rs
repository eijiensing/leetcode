pub struct Solution;

pub fn run() {
    let res = Solution::is_anagram(String::from("anagram"), String::from("nagaram"));
    println!("{res:?}");
    let res = Solution::is_anagram(String::from("rat"), String::from("car"));
    println!("{res:?}");
}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::collections::HashMap;
        let mut m1 = HashMap::<char, u32>::new();
        let mut m2 = HashMap::<char, u32>::new();
        for c in s.chars() {
            m1.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
        for c in t.chars() {
            m2.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
        m1 == m2
    }
}
