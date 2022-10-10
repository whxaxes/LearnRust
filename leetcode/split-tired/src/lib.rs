use std::collections::HashMap;

#[derive(Debug)]
pub struct TiredNode {
    end_of_word: bool,
    map: HashMap<char, Box<TiredNode>>,
}

impl TiredNode {
    pub fn new(end_of_word: bool) -> TiredNode {
        TiredNode {
            end_of_word,
            map: HashMap::new(),
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn split<'a>(info: &'a str, tokens: Vec<&str>) -> Vec<&'a str> {
        let mut result: Vec<&str> = Vec::new();
        let mut tree = Box::new(TiredNode::new(false));

        {
            // 生成字典树
            let mut curr_tree = &mut tree;
            for token in tokens {
                for c in token.chars() {
                    if !curr_tree.map.contains_key(&c) {
                        // 插入新节点
                        curr_tree.map.insert(c, Box::new(TiredNode::new(false)));
                    }

                    curr_tree = curr_tree.map.get_mut(&c).unwrap();
                }

                if token.len() > 0 {
                    // 标记字典结束
                    curr_tree.end_of_word = true;
                }

                curr_tree = &mut tree;
            }
        }

        // 开始匹配
        let mut start = 0; // 匹配游标
        let mut depth = 0; // 匹配深度
        let mut match_tree = &tree;
        for (index, c) in info.chars().enumerate() {
            if match_tree.map.contains_key(&c) {
                match_tree = match_tree.map.get(&c).unwrap();

                if match_tree.end_of_word == true {
                    // 匹配完成
                    result.push(&info[start..(index - depth)]);
                    match_tree = &tree;
                    depth = 0;
                    start = index + 1;
                } else {
                    depth += 1;
                }
            } else {
                match_tree = &tree;
                depth = 0;
            }
        }

        result.push(&info[start..info.len() - depth]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::split("abc---def:ghi::-mno-", vec!["def", "::", "m"]),
            vec!["abc---", ":ghi", "-", "no-"]
        );

        assert_eq!(
            Solution::split("abc---def:ghi::-mno-", vec!["::", "m", "def"]),
            vec!["abc---", ":ghi", "-", "no-"]
        );

        assert_eq!(
            Solution::split("abc---def:ghi::-mno-", vec!["-"]),
            vec!["abc", "", "", "def:ghi::", "mno", ""]
        );

        assert_eq!(
            Solution::split("abc---def:ghi::-mno-", vec!["--", "def", "::", "-"]),
            vec!["abc", "", "", "", ":ghi", "", "mno", ""]
        );
    }
}
