pub struct Solution;

pub fn run() {
    let input = ["eat", "tea", "tan", "ate", "nat", "bat"]
        .iter()
        .map(|e| String::from(*e))
        .collect();
    let res = Solution::group_anagrams(input);
    println!("{res:?}");
}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::BTreeMap;
        let mut maps: BTreeMap<BTreeMap<char, u32>, Vec<String>> = BTreeMap::new();
        for str in strs {
            let mut map = BTreeMap::<char, u32>::new();
            for c in str.chars() {
                map.entry(c).and_modify(|e| *e += 1).or_insert(1);
            }
            maps.entry(map)
                .and_modify(|e| e.push(str.clone()))
                .or_insert(vec![str]);
        }

        maps.values().cloned().collect()
    }
}
