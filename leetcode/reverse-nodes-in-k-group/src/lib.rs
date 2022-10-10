pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}


impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


fn reverse_node(head: Option<Box<ListNode>>, k: i32, in_depth: bool) -> Option<Box<ListNode>> {
  let mut new_head: Option<Box<ListNode>> = None;
  let mut next_head = head;
  let mut i = 0;

  for _ in 0..k {
    // 翻转节点
    if let Some(mut ch) = next_head {
      next_head = ch.next;
      ch.next = new_head;
      new_head = Some(ch);
      i += 1;
    } else {
      break;
    }
  }

  // 发现深度不到 k 值，再颠倒回来
  if in_depth == false && i < k {
    return reverse_node(new_head, k, true);
  } 

  // 如果有 next_head ，递归翻转
  if let Some(r) = next_head {
    next_head = reverse_node(Some(r), k, false);

    // 将尾结点的 next 设置为 next_head
    let mut curr = new_head.as_mut();
    while let Some(h) = curr {
      if let None = h.next {
        h.next = next_head;
        break;
      } else {
        curr = h.next.as_mut();
      }
    }
  }

  new_head
}

impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
      reverse_node(head, k, false)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        fn test_fn(call_list: Vec<i32>, k: i32) -> Vec<i32> {
          let mut head: Option<Box<ListNode>> = None;
          let mut result: Vec<i32> = Vec::new();

          // 生成链表
          for i in 0..call_list.len() {
            let v = call_list.get(call_list.len() - i - 1).unwrap();
            let mut n = Box::new(ListNode::new(*v));
            if let Some(head_node) = head {
              n.next = Some(head_node);
            }

            head = Some(n);
          }

          let mut new_head = Solution::reverse_k_group(head, k);

          // 序列化链表
          let mut curr = new_head.as_mut();
          while let Some(h) = curr {
            result.push(h.val);
            curr = h.next.as_mut();
          }

          result
        }

        assert_eq!(
            test_fn(vec![ 1, 2, 3, 4, 5 ], 2),
            vec![ 2, 1, 4, 3, 5 ]
        );

        assert_eq!(
          test_fn(vec![ 1, 2, 3, 4, 5 ], 3),
          vec![3, 2, 1, 4, 5]
      );
    }
}
