
struct 
Solution {

}
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {

        let mut count = vec![0; 26];

        for ch in s.chars() {
            let idx = (ch as usize) - 97;
            count[idx] = count[idx] + 1;
        }

        for ch in t.chars() {
            let idx = (ch as usize) - 97;
            count[idx] = count[idx] - 1;
        }
    
        for i in 0..25 {
            if count[i] != 0 {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    println!("{}", Solution::is_anagram(String::from("abc"), String::from("cab")));

    println!("{}", Solution::is_anagram(String::from("azdrewq"), String::from("weqzdar")));

    println!("{}", Solution::is_anagram(String::from("zzz"), String::from("zzz")));

    println!("{}", Solution::is_anagram(String::from("zab"), String::from("cab")));
}