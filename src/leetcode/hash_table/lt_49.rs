crate::sln!();

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        let mut categories = HashMap::<Vec<u8>, Vec<usize>>::new();
        let mut flatten_strs = strs.clone();
        unsafe {
            flatten_strs.iter_mut().for_each(|str| {
                str.as_bytes_mut().sort();
            });
        }
        flatten_strs.iter().enumerate().for_each(|(i, str)| {
            categories
                .entry(str.as_bytes().to_owned())
                .or_default()
                .push(i);
        });
        let mut ans = vec![];
        categories.values().for_each(|indexes| {
            let mut sub_ans = vec![];
            indexes
                .iter()
                .for_each(|&i| sub_ans.push(strs[i].to_owned()));
            ans.push(sub_ans)
        });
        ans
    }
}
