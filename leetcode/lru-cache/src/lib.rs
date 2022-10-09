// https://leetcode.cn/problems/lru-cache/

use std::{collections::HashMap};
use std::rc::Rc;
use std::cell::RefCell;

type PureLink = Rc<RefCell<LRUNode>>;
type Link = Option<PureLink>;

pub struct LRUCache {
    map: HashMap<i32, Rc<RefCell<LRUNode>>>,
    head: Link,
    last: Link,
    capacity: i32,
}

struct LRUNode {
    key: i32,
    value: i32,
    prev: Link,
    next: Link,
}

impl LRUNode {
    fn new(key: i32, value: i32) -> Self {
        LRUNode {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        LRUCache {
            capacity,
            head: None,
            last: None,
            map: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let result = self.map.get(&key);
        if let Some(r) = result {
            let result = r.borrow().value;
            self.update_head(&r.clone(), false);
            result
        } else {
            -1
        }
    }
    
    pub fn put(&mut self, key: i32, value: i32) {
        let result = self.map.get(&key);
        if let Some(r) = result {
            r.borrow_mut().value = value;

            // 更新队头
            self.update_head(&r.clone(), false);
        } else {
            let node = Rc::new(RefCell::new(LRUNode::new(key, value)));

            // 更新队头
            self.update_head(&node, true);

            // 写进 map
            self.map.insert(key, node);
        }

        // 检查 map 大小，如果 map 大小超过 cap ，将尾部删掉
        if self.map.len() as i32 > self.capacity {
            self.delete_last();
        }
    }

    fn delete_last(&mut self) {
        if let Some(last_node) = self.last.take() {
            // 从 map 中删掉
            self.map.remove(&last_node.borrow().key);

            // 将 last_node.prev 改为新的 last_node
            if let Some(new_last_node) = last_node.borrow_mut().prev.clone() {
                new_last_node.borrow_mut().next = None;
                self.last = Some(new_last_node);
            }
        }
    }

    fn update_head(&mut self, node: &PureLink, new_insert: bool) {
        // 如果已经存在，将 node 从链表中移除
        let mut lru_node = node.borrow_mut();

        if new_insert == false {
            if let Some(next_node) = &lru_node.next {
                if let Some(prev_node) = &lru_node.prev {
                    next_node.borrow_mut().prev = Some(prev_node.clone());
                }
            }

            if let Some(prev_node) = &lru_node.prev {
                if let Some(next_node) = &lru_node.next {
                    prev_node.borrow_mut().next = Some(next_node.clone());
                } else {
                    // 没有 next ，就是 last
                    prev_node.borrow_mut().next = None;
                    self.last = Some(prev_node.clone());
                }
            }
        } else {
            if let None = self.last {
                self.last = Some(node.clone());
            }
        }

        // 再塞入到头部
        if let Some(head_node) = &self.head {
            // head_node 跟 node 可能是同一个，所以 try borrow 一下
            if let Ok(mut hn) = head_node.try_borrow_mut() {
                if hn.key != lru_node.key {
                    lru_node.prev = None;
                    lru_node.next = Some(head_node.clone());
                    hn.prev = Some(node.clone());
                }
            }
        }

        // 指定为头部
        self.head = Some(node.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        fn test_fn(call_list: Vec<&str>, args_list: Vec<&[i32]>) -> Vec<Option<i32>> {
            let mut lru_cache: Option<LRUCache> = None;
            let mut result: Vec<Option<i32>> = Vec::new();
            
            for (index, method) in call_list.into_iter().enumerate() {
                let args = args_list.get(index).unwrap();
                let first_arg = *args.get(0).unwrap();
                let mut r: Option<i32> = None;

                if method == "LRUCache" {
                    lru_cache = Some(LRUCache::new(first_arg));
                } else {
                    if let Some(cache) = &mut lru_cache {
                        if method == "put" {
                            cache.put(first_arg, *args.get(1).unwrap_or(&0));
                        } else if method == "get" {
                            r = Some(cache.get(first_arg));
                        }
                    }
                }

                result.push(r);
            }

            result
        }

        assert_eq!(
            test_fn(
                vec!["LRUCache", "put","put","get","put","get","put","get","get","get"],
                vec![&[2],&[1,1],&[2,2],&[1],&[3,3],&[2],&[4,4],&[1],&[3],&[4]]
            ),
            vec![None, None, None, Some(1), None, Some(-1), None, Some(-1), Some(3), Some(4)]
        );

        assert_eq!(
            test_fn(
                vec!["LRUCache","put","put","get","put","put","get"],
                vec![&[2],&[2,1],&[2,2],&[2],&[1,1],&[4,1],&[2]]
            ),
            [None, None, None, Some(2), None, None, Some(-1)]
        );

        assert_eq!(
            test_fn(
                vec!["LRUCache","put","put","put","get","put","put","get","put","put","get","put","get","get","get","put","put","get","put","get"],
                vec![&[10],&[7,28],&[7,1],&[8,15],&[6],&[10,27],&[8,10],&[8],&[6,29],&[1,9],&[6],&[10,7],&[1],&[2],&[13],&[8,30],&[1,5],&[1],&[13,2],&[12]]
            ),
            [None, None, None, None, Some(-1), None, None, Some(10), None, None, Some(29), None, Some(9), Some(-1), Some(-1), None, None, Some(5), None, Some(-1)]
        );
    }
}
