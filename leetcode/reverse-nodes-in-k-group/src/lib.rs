use reverse_linked_list::util::ListNode;

pub struct Solution {}

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
          let head = ListNode::convert_vec_to_node(call_list);
          ListNode::convert_node_to_vec(Solution::reverse_k_group(head, k))
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
