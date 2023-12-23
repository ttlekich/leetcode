use std::collections::{BinaryHeap, HashMap};

struct Solution;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            if let Some(existing) = counts.get(&num) {
                counts.insert(num, existing + 1);
            } else {
                counts.insert(num, 1);
            }
        }
        let mut heap = BinaryHeap::new();
        for (k, v) in counts {
            heap.push((v, k))
        }
        let mut result = vec![];
        for _ in 0..k {
            result.push(heap.pop().unwrap().1);
        }
        result
    }
}

fn main() {
    {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let k = 2;
        println!("{:?}", Solution::top_k_frequent(nums, k));
    }
    {
        let nums = vec![1];
        let k = 1;
        println!("{:?}", Solution::top_k_frequent(nums, k));
    }
}
