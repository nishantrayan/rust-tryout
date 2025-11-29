fn main() {
    println!("Hello, world!");
}

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
pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    let mut head = head;
    let mut current = head.as_mut();
    while let Some(curr_node) = current {
        while let Some(next_node) = curr_node.next.as_mut() {
            if next_node.val == curr_node.val {
                curr_node.next = next_node.next.take();
            } else {
                break;
            }
        }
        current = curr_node.next.as_mut();
    }
    head
}
fn listnode_to_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut v = vec![];
    while let Some(node) = list {
        v.push(node.val);
        list = node.next;
    }
    v
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_duplicates_empty() {
        let result = delete_duplicates(None);
        assert_eq!(result, None);
    }

    #[test]
    fn test_delete_duplicates_single() {
        let list = ListNode::new(1);
        let result = delete_duplicates(Some(Box::new(list)));
        assert_eq!(listnode_to_vec(result), [1]);
    }
}
