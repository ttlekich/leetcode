struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];

        let mut pre = 1;
        for i in 0..nums.len() {
            result[i] = pre;
            pre = pre * nums[i];
        }

        let mut post = 1;
        for i in (0..nums.len()).rev() {
            result[i] = post * result[i];
            post = post * nums[i];
        }

        result
    }
}

fn main() {
    {
        let nums = vec![1, 2, 3, 4];
        println!("{:?}", Solution::product_except_self(nums));
    }
    {
        let nums = vec![-1, 1, 0, -3, 3];
        println!("{:?}", Solution::product_except_self(nums));
    }
}
