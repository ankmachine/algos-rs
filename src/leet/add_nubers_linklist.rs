/*
You are given two non-empty linked lists representing two non-negative integers.
The digits are stored in reverse order, and each of their nodes contains a single digit.
Add the two numbers and return the sum as a linked list.
 */

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

pub struct Solution;
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) ->
                                                                                 Option<Box<ListNode>>
    {
        let mut carry: i32 = 0;
        let mut l1 = &l1;
        let mut l2 = &l2;
        let mut output= None;
        let mut curr = &mut output;
        while l1.is_some() || l2.is_some() || carry != 0 {
            let mut sum = carry;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = &node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = &node.next;
            };
            carry = sum/10;
            *curr = Some(Box::new(ListNode::new(sum%10)));
            curr = &mut curr.as_mut().unwrap().next;
        }
        output
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn works() {
        let l1: Vec<i32> = vec![2,4,3];
        let ll1 = ListNode {
            val : 2,
            next : Some(Box::new(ListNode{
                val:4,
                next : Some(Box::new(ListNode::new(3))),
                }))
        };
        let l2: Vec<i32> = vec![5,6,4];
        let ll2 = ListNode {
            val : 5,
            next : Some(Box::new(ListNode{
                val:6,
                next : Some(Box::new(ListNode::new(4)))
            }))
        };
        let output = vec![7, 0, 8];
        let loutput = ListNode {
            val : 7,
            next : Some(Box::new(ListNode{
                val:0,
                next : Some(Box::new(ListNode::new(8)))
            }))
        };
        assert_eq!(Some(Box::new(loutput)),
                   Solution::add_two_numbers(Some(Box::new(ll1)), Some(Box::new(ll2))));
    }
}