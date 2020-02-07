//编写一个函数来查找字符串数组中的最长公共前缀。
//
//如果不存在公共前缀，返回空字符串 ""。
//
//示例 1:
//
//输入: ["flower","flow","flight"]
//输出: "fl"
//示例 2:
//
//输入: ["dog","racecar","car"]
//输出: ""
//解释: 输入不存在公共前缀。
//说明:
//
//所有输入只包含小写字母 a-z 

fn long_common_prefix(arr_str: &Vec<&str>) -> String {
    for s in arr_str {
        if !s.is_ascii() {
            return "".to_string();
        }
    }

    let mut com_pre = String::from("");
    let mut cur_cmp = "";

    for i in 0..arr_str[0].len() {
        cur_cmp = arr_str[0].get(i..i+1).unwrap();
        for j in 0..arr_str.len() {
            if let Some(cur_str) = arr_str[j].get(i..i+1) {
                if cur_str != cur_cmp {
                    return com_pre;
                }
                if j == arr_str.len() - 1 {
                    com_pre += cur_cmp;
                }
            }
        }
    }

    com_pre
}

fn main() {
    assert_eq!("fl", long_common_prefix(&vec!["flower","flow","flight"]));
    assert_eq!("", long_common_prefix(&vec!["dog","racecar","car"]));

    println!("finish");
}
