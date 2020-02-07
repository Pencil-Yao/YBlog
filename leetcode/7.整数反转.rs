//给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。
//
//示例 1:
//
//输入: 123
//输出: 321
// 示例 2:
//
//输入: -123
//输出: -321
//示例 3:
//
//输入: 120
//输出: 21
//注意:
//
//假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [−231,  231 − 1]。请根据这个假设，如果反转后整数溢出那么就返回 0。
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

fn main() {
    assert_eq!(321, reverse(123));
    assert_eq!(-321, reverse(-123));
    assert_eq!(21, reverse(120));
    assert_eq!(0, reverse(2147483647));
    assert_eq!(0, reverse(-2147483648));
    assert_eq!(2147483641, reverse(1463847412));
    assert_eq!(0, reverse(1463847422));

    println!("finish");
}
