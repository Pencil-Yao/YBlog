use std::collections::HashMap;

type MaxPair = (i32, char);

struct Solution {

}

impl Solution {
    fn longest_length_of_string(s: String) -> i32 {
        let mut record_map = HashMap::new();
        let mut max_pair: MaxPair = (0, ' ');
        let mut idx = 0;
        for x in s.chars() {
            idx += 1;
            match record_map.get(&x) {
                Some(i) => {
                   let num = count_num(&record_map, i);
                    if num > max_pair.0 {
                        max_pair.0 = num;
                        max_pair.1 = x;
                    }
                    record_map.insert(x, idx);
                }
                None => {record_map.insert(x, idx);}
            };
        }
        max_pair.0
    }
}

fn count_num(map: &HashMap<char,i32>, threshold: &i32) -> i32 {
    let mut count_number = 0;
    for (_liter, index) in map.iter() {
        if index >= threshold {
            count_number += 1;
        }
    };
    return count_number;
}

fn main() {
    assert_eq!(3, Solution::longest_length_of_string("abcabcbb".to_string()));
    assert_eq!(1, Solution::longest_length_of_string("bbbbb".to_string()));
    assert_eq!(3, Solution::longest_length_of_string("pwwkew".to_string()));
    println!("finish!");
}

