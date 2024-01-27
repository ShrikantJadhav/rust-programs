
struct Solution {

}

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let mut v :Vec<i32>= vec![0; nums.len() * 2];

        for (i, &val) in nums.iter().enumerate() {
            v[i] = val;
            v[i + nums.len()] = val;
        }
        return v;
    }
}

fn main() {

    let s = Solution{};

    println!("{:?}", Solution::get_concatenation(vec![1,2,3]));
}
