//! 题目：
//! 给定一个含有 n 个正整数的数组和一个正整数 target。找出该数组中满足其和 ≥ target 的长度最小的连续子数组 [numsl, numsl+1, ..., numsr-1, numsr]，并返回其长度。
//! 如果不存在符合条件的子数组，返回 0。
//! 
//! 思路：
//! 这一题的解题思路是用滑动窗口。在滑动窗口 [i,j] 之间不断往后移动，如果总和小于 s，就扩大右边界 right，不断加入右边的值，直到 sum >= s，然后判断满足的子数组的长度，
//! 再缩小 left 左边界，直到 sum < s，这时候右边界又可以往右移动，寻找下一个满足的子数组。

fn min(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    }

    b
}

pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut left = 0usize;
    let mut sum = 0;
    let mut res = nums.len() as i32 + 1;

    for right in 0..nums.len() {
        let num = nums[right];
        sum += num;
        while sum >= target {
            res = min(res, (right - left + 1) as i32);
            sum -= nums[left];
            left += 1;
        }
    }

    if res == nums.len() as i32 + 1 {
        return 0;
    }

    res
}

#[cfg(test)]
mod tests{
    use super::min_sub_array_len;

    #[test]
    fn min_sub_array_len_test() {
        let nums = vec![2,3,1,2,4,3];
        let len = min_sub_array_len(7, nums);
        assert_eq!(len, 2);
    }

}