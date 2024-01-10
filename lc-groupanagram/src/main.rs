use std::collections::HashMap;

struct Solution {

}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        
        let mut hm = HashMap::new();
        for str in strs {
            let mut key: Vec<char> = str.chars().collect();
            key.sort();
            hm.entry(key).or_insert(vec![]).push(str);
        }

        hm.values().cloned().collect() 
    }
}

fn main() {
    println!("{:?}", Solution::group_anagrams(vec![String::from("abc"), String::from("bca"), String::from("cba"), String::from("abcsd")]));

    println!("{:?}", Solution::group_anagrams(vec![String::from("lkm"), String::from("bca"), String::from("cba"), String::from("mlk")]));

    println!("{:?}", Solution::group_anagrams(vec![String::from("pop"), String::from("bob"), String::from("kob"), String::from("bok")]));
}