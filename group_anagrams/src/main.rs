use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<Vec<char>, Vec<String>> = HashMap::new();

        for s in strs {
            let mut sorted = s.chars().collect::<Vec<char>>();
            sorted.sort();
            groups.entry(sorted).or_insert(vec![]).push(s);
        }

        groups.values().cloned().collect()
    }
}

struct Solution;
fn main() {
    dbg!(Solution::group_anagrams(
        vec!["eat", "tea", "tan", "ate", "nat", "bat"]
            .iter()
            .map(|el| el.to_string())
            .collect()
    ));
}
