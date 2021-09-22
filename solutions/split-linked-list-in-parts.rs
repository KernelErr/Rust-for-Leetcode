// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut ans: Vec<Option<Box<ListNode>>> = Vec::new();
        let mut n: i32 = 0;
        let mut cur = &head;
        while cur.is_some() {
            n += 1;
            cur = &cur.as_ref().unwrap().next;
        }
        let m = n / k;
        let mut r = n % k;
        let mut cur = head;
        for _ in 0..k { // k parts
            let mut cnt = m;
            if r > 0 {
                cnt += 1;
                r -= 1;
            }
            let mut head = ListNode::new(0);
            let mut tail = &mut head;
            for _ in 0..cnt {
                cur = match cur.take() {
                    Some(x) => {
                        let val = x.val;
                        tail.next = Some(Box::new(ListNode::new(val)));
                        x.next
                    },
                    None => {
                        tail.next = None;
                        None
                    }
                };
                tail = tail.next.as_mut().unwrap();
            }
            ans.push(head.next);
        }
        ans
    }
}