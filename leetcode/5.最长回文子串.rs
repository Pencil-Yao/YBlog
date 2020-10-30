//给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。
//
//示例 1：
//
//输入: "babad"
//输出: "bab"
//注意: "aba" 也是一个有效答案。
//示例 2：
//
//输入: "cbbd"
//输出: "bb"

use std::cmp::max;

type MaxStr = (usize, String);
struct Solution {

}

impl Solution {
    fn longest_palindrome(s:&String) -> String {
        let mut max_str:MaxStr = (0, "".to_string());

        for oi in 0..s.len() {
            for interval in 1..(oi+1) {
                if !judge_char_equal(s, oi-interval, oi+interval) {
                    break;
                }
                if 2*interval + 1 > max_str.0 {
                    max_str.0 = 2*interval + 1;
                    max_str.1 = s.get((oi-interval)..(oi+interval+1)).unwrap().to_string();
                }
            }
        }

        for oi in 0..s.len()-1 {
            if !judge_char_equal(s, oi, oi+1) {
                continue;
            }
            for interval in 1..(oi+1) {
                if !judge_char_equal(s, oi-interval, oi+interval+1) {
                    break;
                }
                if 2*interval + 2 > max_str.0 {
                    max_str.0 = 2*interval + 2;
                    max_str.1 = s.get((oi-interval)..(oi+interval+2)).unwrap().to_string();
                }
            }
        }

        max_str.1
    }
}

fn judge_char_equal(s:&String, i:usize, j:usize) -> bool {
    if max(i, j) >= s.len() {
        return false;
    }
    s.get(i..i+1) == s.get(j..j+1)
}

fn main() {
    assert_eq!("abacaba", Solution::longest_palindrome(&"abacabac".to_string()));
    assert_eq!("abaccaba", Solution::longest_palindrome(&"abaccabac".to_string()));
    println!("{}", "finish");
}
