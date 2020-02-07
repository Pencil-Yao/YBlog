//罗马数字包含以下七种字符: I， V， X， L，C，D 和 M。
//
//字符          数值
//I             1
//V             5
//X             10
//L             50
//C             100
//D             500
//M             1000
//例如， 罗马数字 2 写做 II ，即为两个并列的 1。12 写做 XII ，即为 X + II 。 27 写做  XXVII, 即为 XX + V + II 。
//
//通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。数字 1 在数字 5 的左边，所表示的数等于大数 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况：
//
//I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
//X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。 
//C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
//给定一个罗马数字，将其转换成整数。输入确保在 1 到 3999 的范围内。
//
//示例 1:
//
//输入: "III"
//输出: 3
//示例 2:
//
//输入: "IV"
//输出: 4
//示例 3:
//
//输入: "IX"
//输出: 9
//示例 4:
//
//输入: "LVIII"
//输出: 58
//解释: L = 50, V= 5, III = 3.
//示例 5:
//
//输入: "MCMXCIV"
//输出: 1994
//解释: M = 1000, CM = 900, XC = 90, IV = 4.

use std::collections::HashMap;

fn reverse_roman(mut roman_str: String) -> i32 {
    let roman_num: Vec<(&str, i32)> = vec![("M", 1000), ("CM", 900), ("D", 500), ("CD", 400), ("C", 100),
                                           ("XC", 90), ("L", 50), ("XL", 40), ("X", 10),
                                           ("IX", 9), ("V", 5), ("IV", 4), ("I", 1)];
    let mut res_num = 0;
    let mut roman_num_map = HashMap::new();
    
    for (rn, an) in roman_num.iter() {
        roman_num_map.insert(*rn, *an);
    }

    while roman_str.len() != 0 {

        if roman_str.len() > 1 {
            if let Some(cur_str) = roman_str.get(roman_str.len()-2..roman_str.len()) {
                if roman_num_map.contains_key(cur_str) {
                    res_num += roman_num_map[cur_str];
                    roman_str.pop();
                    roman_str.pop();
                    continue;
                }
            }
        }

        if let Some(cur_str) = roman_str.get(roman_str.len()-1..roman_str.len()) {
            if roman_num_map.contains_key(cur_str) {
                res_num += roman_num_map[cur_str];
                roman_str.pop();
                continue;
            } else { panic!("error input: {}", cur_str); }
        }
    }
    res_num
}

fn main() {
    assert_eq!(4, reverse_roman("IV".to_string()));
    assert_eq!(3, reverse_roman("III".to_string()));
    assert_eq!(9, reverse_roman("IX".to_string()));
    assert_eq!(58, reverse_roman("LVIII".to_string()));
    assert_eq!(1994, reverse_roman("MCMXCIV".to_string()));

    println!("finish");
}
