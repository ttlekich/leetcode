use std::collections::HashSet;

fn main() {
    let example_1 = vec![1, 2, 3, 1];
    let output_1 = contains_duplicate(example_1);
    println!("{}", output_1);

    let example_2 = vec![1, 2, 3, 4];
    let output_2 = contains_duplicate(example_2);
    println!("{}", output_2);

    let example_3 = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
    let output_3 = contains_duplicate(example_3);
    println!("{}", output_3);
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let set: HashSet<i32> = nums.clone().into_iter().collect();
    set.len() != nums.len()
}
