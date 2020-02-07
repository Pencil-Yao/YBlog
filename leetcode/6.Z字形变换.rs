//将一个给定字符串根据给定的行数，以从上往下、从左到右进行 Z 字形排列。
//
//比如输入字符串为 "LEETCODEISHIRING" 行数为 3 时，排列如下：
//
//L   C   I   R
//E T O E S I I G
//E   D   H   N
//之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："LCIRETOESIIGEDHN"。
//
//请你实现这个将字符串进行指定行数变换的函数：
//
//string convert(string s, int numRows);
//示例 1:
//
//输入: s = "LEETCODEISHIRING", numRows = 3
//输出: "LCIRETOESIIGEDHN"
//示例 2:
//
//输入: s = "LEETCODEISHIRING", numRows = 4
//输出: "LDREOEIIECIHNTSG"
//解释:
//
//L     D     R
//E   O E   I I
//E C   I H   N
//T     S     G

fn convert(s:&String, n: usize) -> String {
    let mut res = "".to_string();
    for i in 0..n {
       for j in 0..=(s.len()/(2*n-2)) {
           if let Some(c1) = s.get(j*(2*n-2)+i..j*(2*n-2)+i+1) {
               res.push_str(c1);
           }
           if i % (n-1) != 0 {
               if let Some(c2) = s.get((j+1)*(2*n-2)-i..(j+1)*(2*n-2)-i+1) {
                   res.push_str(c2);
               }
           }
       }
    }
    res
}

fn main() {
    let input = "LEETCODEISHIRING".to_string();
    let num_row = 3;
    assert_eq!("LCIRETOESIIGEDHN", convert(&input, num_row));

    let input = "LEETCODEISHIRING".to_string();
    let num_row = 4;
    assert_eq!("LDREOEIIECIHNTSG", convert(&input, num_row));

    println!("finish");
}
