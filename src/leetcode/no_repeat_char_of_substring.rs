//! 思路：
//! 使用滑动窗口，它的右边界不断的右移，只要没有重复的字符，就持续向右扩大窗口边界。一旦探测到出现了重复字符，就需要右边界先停下，然后缩小左边界，
//! 直到重复的字符移出了左边界，然后继续移动滑动窗口的右边界。以此类推，每次移动需要计算当前长度，并判断是否需要更新最大长度，最终最大的值就是题目中的所求。

fn max(a: i32, b: i32) -> i32 {
    if a >= b {
        a
    }else {
        b
    }
}

/// 无重复字符的最长子串
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut result = 0;
    let mut left = 0i32;
    let mut right = -1i32;
    let mut freg = [0; 127];
    let s = s.into_bytes();

    while (left as usize) < s.len() {
        let right_index = (right + 1) as usize;
        if right_index < s.len() && freg[s[right_index] as usize] == 0 {
            right += 1;
            freg[s[right_index] as usize] += 1;
        }else {
            freg[s[left as usize] as usize] -= 1;
            left += 1;
        }
        result = max(result, right - left + 1);
    }
    
    result
}


#[cfg(test)]
mod tests{
    use super::length_of_longest_substring;

    #[test]
    fn length_of_longest_substring_test() {
        let num = length_of_longest_substring("abcabcbb".to_owned());
        assert_eq!(num, 3);
    }
}