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

    pub fn create_tree(tokens: &Vec<&str>) -> Box<TiredNode> {
        let mut tree = Box::new(TiredNode::new(false));

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

        tree
    }
}

pub struct Solution {}

impl Solution {
    pub fn split<'a>(info: &'a str, tokens: Vec<&str>) -> Vec<&'a str> {
        let mut result: Vec<&str> = Vec::new();
        let tree = TiredNode::create_tree(&tokens);

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


    // 深度优先
    pub fn split_deep_first<'a>(info: &'a str, tokens: Vec<&str>) -> Vec<&'a str> {
        let mut result: Vec<&str> = Vec::new();
        let tree = TiredNode::create_tree(&tokens);

        // 开始匹配
        let mut start = 0; // 匹配游标
        let mut depth: usize = 0; // 匹配深度
        let mut match_tree = &tree;
        let mut match_record: Option<(usize, usize)> = None;

        let mut index = 0;
        while index < info.len() {
            let c = info.chars().nth(index).unwrap();

            if match_tree.map.contains_key(&c) {
                match_tree = match_tree.map.get(&c).unwrap();

                // 匹配到了，保存当前记录，然后继续尝试深入匹配
                if match_tree.end_of_word == true {
                    match_record = Some((index, depth));
                }

                depth += 1;
                index += 1;

                // 最后一次还要继续往下 check 一下
                // 因为可能继续尝试到最后一个字符就跳出循环了
                if index < info.len() {
                    continue;
                }
            }

            // 此时代表要么当前字符不匹配，要么到了最后一个字符，检查此前有无匹配的，如果有回溯到此前的匹配
            if let Some((old_index, old_depth)) = match_record {
                // 匹配完成
                result.push(&info[start..(old_index - old_depth)]);
                start = old_index + 1;
                index = old_index + 1;
                match_record = None;
            } else {
                index += 1;
            }

            match_tree = &tree;
            depth = 0;
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

    #[test]
    fn it_deep_first() {
        assert_eq!(
            Solution::split_deep_first("abc---def:ghi::-mno-", vec!["def", "::", "m"]),
            vec!["abc---", ":ghi", "-", "no-"]
        );

        assert_eq!(
            Solution::split_deep_first("abc---def:ghi::-mno-", vec!["::", "m", "def"]),
            vec!["abc---", ":ghi", "-", "no-"]
        );

        assert_eq!(
            Solution::split_deep_first("abc---def:ghi::-mno-", vec!["-"]),
            vec!["abc", "", "", "def:ghi::", "mno", ""]
        );

        assert_eq!(
            Solution::split_deep_first("abc---def:ghi::-mno-", vec!["--", "def", "::", "-"]),
            vec!["abc", "", "", ":ghi", "", "mno", ""]
        );

        assert_eq!(
            Solution::split_deep_first("abc---def:ghi::-mno-", vec!["--", ":", "::", "-"]),
            vec!["abc", "", "def", "ghi", "", "mno", ""]
        );
    }
}
