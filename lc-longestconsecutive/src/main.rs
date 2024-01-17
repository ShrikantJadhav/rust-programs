struct Solution {

}

impl Solution {
    pub fn longest_consecutive(mut nums: Vec<i32>) -> i32 {

        if nums.len() == 0 {
            return 0;
        }

        nums.sort();

        let mut l = 1;
        let mut m = 1;
        for i in 1..nums.len() {
            if nums[i-1] + 1 == nums[i] {
                l+=1;
                if l > m {
                    m = l;
                }
            } else if nums[i-1] == nums[i] {
                continue;
            } else {
                l = 1;
            }
        }
        return m
    }
}

fn main() {
    println!("{:?}", Solution::longest_consecutive(vec![0,1,2,3,4]));

    println!("{:?}", Solution::longest_consecutive(vec![10,9,8,7,6]));

    println!("{:?}", Solution::longest_consecutive(vec![0,11,2,3,4]));

    println!("{:?}", Solution::longest_consecutive(vec![10,11,3,4, 12]));
}