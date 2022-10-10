#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn convert_vec_to_node(list: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;

        // 生成链表
        for i in 0..list.len() {
            let v = list.get(list.len() - i - 1).unwrap();
            let mut n = Box::new(ListNode::new(*v));
            if let Some(head_node) = head {
                n.next = Some(head_node);
            }

            head = Some(n);
        }

        head
    }

    pub fn convert_node_to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        // 序列化链表
        let mut curr = head.as_mut();
        while let Some(h) = curr {
            result.push(h.val);
            curr = h.next.as_mut();
        }

        result
    }
}
