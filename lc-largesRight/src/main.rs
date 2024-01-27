struct Solution {

}

impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        
        let n = arr.len();

        let mut max_value = -1;
        let mut max_to_right: Vec<i32> = vec![-1; n];

        for i in (0..n
        ).rev() {
            max_to_right[i] = max_value;
            println!("{:?}",arr[i]);
            if arr[i] > max_value {
                max_value = arr[i];
            }
        }
        return max_to_right;
    }
}

fn main() {
    println!("{:?}", Solution::replace_elements(vec![1,2,3,4,5,6]));

    println!("{:?}", Solution::replace_elements(vec![7,5,6,4,6,3,9]));
    
}
