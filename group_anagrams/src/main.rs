use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups: HashMap<Vec<char>, Vec<String>> = HashMap::new();
        let normalized = strs.iter().map(|s| {
            let mut sorted = s.chars().collect::<Vec<char>>();
            sorted.sort();
            (sorted, s.clone())
        });

        for item in normalized {
            if let Some(existing) = groups.get_mut(&item.0) {
                existing.push(item.1);
            } else {
                groups.insert(item.0, vec![item.1]);
            }
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
