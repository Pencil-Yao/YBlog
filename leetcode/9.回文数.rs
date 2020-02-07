//判断一个整数是否是回文数。回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。
//
//示例 1:
//
//输入: 121
//输出: true
//示例 2:
//
//输入: -121
//输出: false
//解释: 从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
//示例 3:
//
//输入: 10
//输出: false
//解释: 从右向左读, 为 01 。因此它不是一个回文数。
//进阶:
//
//你能不将整数转为字符串来解决这个问题吗？

fn pow(a: u32, b: i32) -> u32 {
    if b == 0 {
        return 1;
    }
    let mut res = 1;
    for _i in 0..b {
        res *= a;
    }
    res
}

fn abs_i32(i: i32) -> u32 {
    if i < 0 {
        return (-(i+1) as u32) + 1;
    }
    i as u32
}


fn reverse(num: i32) -> i32 {
    let mut res: u32 = 0;
    let mut abs_num: u32 = abs_i32(num);
    let n_neg: bool = num.ge(&0);

    let threshold:u32 = pow(2, 31)-(n_neg as u32);

    let mut all_num: Vec<u32> = vec![];
    let mut level = -1;
    while abs_num.ne(&0) {
        let cur_num = abs_num % 10;
        abs_num = abs_num / 10;
        all_num.push(cur_num);
        level += 1;
    }

    for i in 0..all_num.len() {
        if level >= 9 && all_num[i] > 2 {
            return 0;
        }
        res += all_num[i] * pow(10, level);

        if res > threshold {
            return 0;
        }

        level -= 1;
    }

    if !n_neg {
        return (res as i32)*-1;
    }
    res as i32
}

fn is_palindrome(num: i32) -> bool {

    if num < 0 {
        return false;
    }

    if num == reverse(num) {
        return true;
    }

    false
}

fn main() {
    assert_eq!(true, is_palindrome(0));
    assert_eq!(true, is_palindrome(121));
    assert_eq!(true, is_palindrome(2147447412));

    assert_eq!(false, is_palindrome(2147447413));
    assert_eq!(false, is_palindrome(-121));

    println!("finish");
}
