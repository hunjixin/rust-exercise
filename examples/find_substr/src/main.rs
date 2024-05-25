use std::{collections::HashSet, hash::Hash, ops::Index};

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut sub_string = String::new();
    let mut max_no_dup_str_len = 0i32;
    for char in s.chars() {
        match sub_string.find(char) {
            Some(last_index) => {
                max_no_dup_str_len = max_no_dup_str_len.max(sub_string.len() as i32);
                sub_string = sub_string.split_off(last_index + 1);
                sub_string.push(char)
            }
            None => sub_string.push(char),
        }
    }
    max_no_dup_str_len.max(sub_string.len() as i32)
}

pub fn length_of_longest_substring_v2(s: String) -> i32 {
    let mut max_no_dup_str_len = 0i32;

    let mut hash_set: Vec<char> = vec![];
    let mut right = 0usize;
    for char in s.chars() {
        while hash_set.contains(&char) {
            hash_set.remove(0);
        }
        hash_set.push(char);
        max_no_dup_str_len = max_no_dup_str_len.max((hash_set.len()) as i32)
    }
    max_no_dup_str_len
}

fn main() {
    assert_eq!(3i32, length_of_longest_substring_v2("abcabcbb".to_string()));
    assert_eq!(1i32, length_of_longest_substring_v2("bbbbbb".to_string()));
    assert_eq!(3i32, length_of_longest_substring_v2("pwwkew".to_string()));
    assert_eq!(0i32, length_of_longest_substring_v2("".to_string()));
    assert_eq!(1i32, length_of_longest_substring_v2(" ".to_string()));
}
