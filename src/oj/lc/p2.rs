struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut dummy = ListNode::new(0);
        let mut cur = &mut dummy;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry > 0 {
            let mut sum = carry;

            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next;
            }

            cur.next = Some(Box::new(ListNode::new(sum % 10)));
            carry = sum / 10;

            if let Some(ref mut next_node) = cur.next {
                cur = next_node;
            } else {
                break;
            }
        }

        dummy.next
    }
}