use std::collections::HashMap;

struct Solution{

}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut mInd = HashMap::<i32, i32>::new();
        
        for i in 0..nums.len() {
            let res = target - nums[i];
            match mInd.get(&res) {
                Some(&v) => return vec![i as i32, v],
                None => mInd.insert(nums[i], i as i32),
            };
        }
        return vec![-1,-1]
    }
}

fn main() {

    println!("{:?}", Solution::two_sum(vec![1,9,2,3,1,4], 11));

    println!("{:?}", Solution::two_sum(vec![1,2,4,3,2,1], 5));

    println!("{:?}", Solution::two_sum(vec![1,9,1,6,9,4], 15));
}