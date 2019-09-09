// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// 0ms 2mb
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        Some(mut node) => {
            let mut value = node.val;
            let mut ptr = &mut node;

            while ptr.next.is_some() {
                if ptr.next.as_ref().map_or(false, |n| value == n.val) {
                    ptr.next = ptr.next.as_mut().and_then(|n| n.next.take())
                } else {
                    value = ptr.next.as_ref().unwrap().val;
                    ptr = ptr.next.as_mut().unwrap();
                }
            }
            Some(node)
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut first = ListNode::new(1);
        let mut second = ListNode::new(1);
        let third = ListNode::new(2);

        second.next = Some(Box::new(third));
        first.next = Some(Box::new(second));

        let mut res = ListNode::new(1);
        res.next = Some(Box::new(ListNode::new(2)));

        println!("first: {:?}", first);
        println!("res: {:?}", res);

        assert_eq!(
            delete_duplicates(Some(Box::new(first))),
            Some(Box::new(res))
        );
    }
}
