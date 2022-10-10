// Definition for singly-linked list.
pub mod util;
use util::ListNode;

pub struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head: Option<Box<ListNode>> = None;
        let mut next_head = head;

        // 翻转节点
        while let Some(mut ch) = next_head {
            next_head = ch.next;
            ch.next = new_head;
            new_head = Some(ch);
        }

        new_head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        fn test_fn(call_list: Vec<i32>) -> Vec<i32> {
            let head = ListNode::convert_vec_to_node(call_list);
            ListNode::convert_node_to_vec(Solution::reverse_list(head))
          }
  
          assert_eq!(
              test_fn(vec![ 1, 2, 3, 4, 5 ]),
              vec![ 5, 4, 3, 2, 1 ]
          );

          assert_eq!(
            test_fn(vec![]),
            vec![]
        );
    }
}
