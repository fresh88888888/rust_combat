//! 题目：
//! DNA 序列 由一系列核苷酸组成，缩写为 'A', 'C', 'G' 和 'T'；给定一个表示 DNA 序列 的字符串 s ，返回所有在 DNA 分子中出现不止一次的 长度为 10 的序列 (子字符串)。你可以按 任意顺序 返回答案
//! 解法：
//! O(n) 的时间复杂度，一次遍历将长度为 10 的所有子串放在 map 中，并且计数，最后将出现次数大于 1 的子串找出来返回。

use std::collections::HashMap;


pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    let sub_strlen = 10;

    if s.len() <= sub_strlen {
        return vec![];
    }

    let s = s.into_bytes();
    let end = s.len() - 10;
    let mut result = HashMap::new();

    for i in 0..=end  {
        let tmp1 = &s[i..i + sub_strlen];
        let tmp1 = unsafe {String::from_utf8_unchecked(tmp1.to_vec())};
        result.entry(tmp1).and_modify(|count| *count += 1).or_insert(1);
    }

    let result: HashMap<&String, &i32> = result.iter().filter(|(_, count)| **count > 1).collect();

    result.into_keys().map(|key| key.to_owned()).collect::<Vec<String>>()
}

#[cfg(test)]
mod tests{
    use crate::leetcode::find_repeated_dna_sequences;

    #[test]
    fn find_repeated_dna_sequences_test() {
        let s  = "AAAAAAAAAAAAA".to_owned();
        let result = find_repeated_dna_sequences(s);
        println!("{:?}", result);
        assert_eq!(result, vec!["AAAAAAAAAA"])
    }
}