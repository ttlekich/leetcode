fn main() {
    {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        println!("{:?}", two_sum(nums, target))
    }
    {
        let nums = vec![3, 2, 4];
        let target = 6;
        println!("{:?}", two_sum(nums, target))
    }
    {
        let nums = vec![3, 3];
        let target = 6;
        println!("{:?}", two_sum(nums, target))
    }
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let indexes = 0..nums.len();

    let mut nums: Vec<(&i32, usize)> = nums.iter().zip(indexes).collect();

    nums.sort();

    let mut i = 0;
    let mut j = nums.len() - 1;

    loop {
        let low = nums[i];
        let high = nums[j];

        if low.0 + high.0 == target {
            return vec![low.1.clone() as i32, high.1.clone() as i32];
        }

        if low.0 + high.0 < target {
            i += 1;
            continue;
        }

        if low.0 + high.0 > target {
            j -= 1;
            continue;
        }
    }
}
