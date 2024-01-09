use std::collections::HashSet;

struct Solution {

}
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        
        let mut arr = HashSet::new();

        for elem in nums.iter(){
            if arr.contains(elem) {
                return true;
            }
            arr.insert(elem);
        }
        return false;
    }
}

fn main() {

    println!("{}", Solution::contains_duplicate(vec![1,2,3]));

    println!("{}", Solution::contains_duplicate(vec![1,1,2,3]));

    println!("{}", Solution::contains_duplicate(vec![1,2,3,1]));

    println!("{}", Solution::contains_duplicate(vec![1,2,3,2]));
}