//! 题目：
//! 给定一个字符串 s 和一些 长度相同 的单词 words 。找出 s 中恰好可以由 words 中所有单词串联形成的子串的起始位置。注意子串要与 words 中的单词完全匹配，
//! 中间不能有其他字符 ，但不需要考虑 words 中单词串联的顺序。
//! 
//! 解法：
//! 要点：1.字符串数组里面的字符串长度都是一样的；2.要求字符串数组中的字符串都要连续连在一起的，前后顺序可以是任意排列组合；
//! 假设，words 的长度是 n，words 中每个单词的长度是 w，所以我们每次可以从字符串 s 取 n * w 长度来判断它是否是 words 的自由排列组合。
//! 
//! 在判断时，我们先将 words 中每个单词的数量放在一个计数器中，可以用 map 表示，例如：["aaa", "bbb"] 表示成 {"aaa": 1, "bbb": 1}；["aaa", "aaa", "ccc"] 
//! 表示成 {"aaa": 2, "ccc": 1}；然后我们将 s 中每个 n * w 长度的子串每 w 个一组放在一个 map 中计数，最后和 words 的计数器比较是否相等，如果相等就是我们想要的子串。

use std::collections::HashMap;


pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    
    let mut result: Vec<i32> = Vec::new();
    let n = words.len();
    if n == 0 {
        return result;
    }

    let w = words[0].len();
    if s.len() < n * w {
        return result;
    }

    let mut counter = HashMap::new();
    for i in 0..n {
        counter.entry(words[i].as_str()).and_modify(|count| *count += 1).or_insert(1);
    }

    for i in 0..= (s.len() - n * w) {
        let mut tmp_counter = HashMap::new();
        for j in 0..n {
            let start = i + j * w;
            let tmp_str = &s[start..(start + w)];
            tmp_counter.entry(tmp_str).and_modify(|count| *count += 1).or_insert(1);
        }

        if tmp_counter == counter {
            result.push(i as i32);
        }
    }

    result

}

#[cfg(test)]
mod tests{
    use super::find_substring;

    #[test]
    fn find_substring_test() {
        let s = "barfoofoobarthefoobarman".to_owned();
        let words= vec!["bar","foo","the"].iter().map(|&s| s.into()).collect();
        let nums = find_substring(s, words);
        assert_eq!(nums, vec![6,9,12])
    }
}