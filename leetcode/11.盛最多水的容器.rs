//给定 n 个非负整数 a1，a2，...，an，每个数代表坐标中的一个点 (i, ai) 。在坐标内画 n 条垂直线，垂直线 i 的两个端点分别为 (i, ai) 和 (i, 0)。找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。
//
//说明：你不能倾斜容器，且 n 的值至少为 2。
//
//
//
//图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。
//
// 
//
//示例:
//
//输入: [1,8,6,2,5,4,8,3,7]
//输出: 49

use std::cmp::min;

fn abs_i32(mut num: i32) -> i32 {

    if num < 0 {
        num = -num;
    }
    num
}

fn max_area(num_vec: Vec<i32>) -> i32 {
    let mut max_ar = 0;
    let mut mid_num: Vec<(i32, i32)> = vec![];
    for i in 0..num_vec.len() {
        mid_num.push((num_vec[i], i as i32));
    }
    mid_num.sort_by(|(v1, _r1), (v2, _r2)| v2.cmp(v1));

    let mut idx = 1;
    for (v, r) in mid_num.iter() {
        for i in idx..mid_num.len() {

            let cur_area = min(*v, mid_num[i].0) * abs_i32(r - mid_num[i].1);
            if cur_area > max_ar {
                max_ar = cur_area;
            }
        }
        idx += 1;
    }
    max_ar
}

fn main() {
    let test_array = vec![1,8,6,2,5,4,8,3,7];

    println!("{}", max_area(test_array));
}
