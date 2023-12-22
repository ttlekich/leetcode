use std::collections::HashMap;

fn main() {
    {
        let s = "anagram".to_string();
        let t = "nagaram".to_string();
        println!("{}", is_anagram(s, t));
    }
    {
        let s = "rat".to_string();
        let t = "car".to_string();
        println!("{}", is_anagram(s, t));
    }
}

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_count = HashMap::new();

    for c in s.chars() {
        s_count.insert(
            c,
            if s_count.contains_key(&c) {
                s_count[&c] + 1
            } else {
                1
            },
        );
    }

    for c in t.chars() {
        if let Some(existing) = s_count.get(&c) {
            if *existing == 1 {
                s_count.remove(&c);
            } else {
                s_count.insert(c, s_count[&c] - 1);
            }
        } else {
            return false;
        }
    }

    return true;
}
