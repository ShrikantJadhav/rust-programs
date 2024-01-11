use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

struct Solution {

}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        
        let mut hm = HashMap::new();

        for num in nums {
            hm.entry(num).and_modify(|entry| *entry += 1).or_insert(0);
        }

        let mut min_heap = BinaryHeap::new();
        for (i, f) in hm {
            min_heap.push(Reverse((f,i)));

            if min_heap.len() > k as usize {
                min_heap.pop();
            }
        }

        min_heap
            .into_iter()
            .map(|Reverse((v, k))| k)
            .collect::<Vec<i32>>()
    }
}

fn main() {

    let mut tk =  Solution::top_k_frequent(vec![1,1,1,2,3,3,1], 2);
    println!("{:?}", tk);

    tk =  Solution::top_k_frequent(vec![1,1,1,2,2,2,2,3,3,1], 2);
    println!("{:?}", tk);


    tk =  Solution::top_k_frequent(vec![10,1,12,2,32,3,1], 3);
    println!("{:?}", tk);


    tk =  Solution::top_k_frequent(vec![1,1,1,20,1], 2);
    println!("{:?}", tk);


}