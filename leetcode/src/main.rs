fn main() {
    println!("Run tests with: cargo test");
}

// Helper function to convert ListNode to Vec for testing
fn listnode_to_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut v = vec![];
    while let Some(node) = list {
        v.push(node.val);
        list = node.next;
    }
    v
}

// ============================================================================
// LeetCode Solutions
// ============================================================================

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return vec![j as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let (mut n, mut rev) = (x, 0);
    while n != 0 {
        rev = rev * 10 + n % 10;
        n /= 10;
    }
    rev == x
}

pub fn roman_to_int(s: String) -> i32 {
    let s = String::from(s + " ");
    let chars: Vec<char> = s.chars().collect::<Vec<char>>();
    let mut windows = chars.windows(2);
    let mut num = 0;
    while let Some(window) = windows.next() {
        let (first, second) = window.split_at(1);
        let add_num = match (first[0], second[0]) {
            ('I', 'V') | ('I', 'X') => -1,
            ('I', _) => 1,
            ('X', 'L') | ('X', 'C') => -10,
            ('X', _) => 10,
            ('C', 'D') | ('C', 'M') => -100,
            ('C', _) => 100,
            ('V', _) => 5,
            ('L', _) => 50,
            ('D', _) => 500,
            ('M', _) => 1000,
            _ => 0,
        };
        num += add_num;
    }
    num
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let mut index: usize = 0;
    loop {
        let all_chars: Vec<_> = strs
            .iter()
            .map(|s| s.chars().nth(index))
            .collect();

        if all_chars.iter().any(|c| c.is_none()) {
            break;
        }

        if let Some(first) = all_chars.first() {
            let all_match = all_chars.iter().all(|c| c == first);
            if all_match {
                index += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    strs.first()
        .unwrap()
        .chars()
        .take(index)
        .collect::<String>()
}

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => { stack.push(c); }
            ')' => if stack.pop() != Some('(') { return false; }
            ']' => if stack.pop() != Some('[') { return false; }
            '}' => if stack.pop() != Some('{') { return false; }
            _ => { return false; }
        }
    }
    stack.is_empty()
}

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

pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut new_list = Box::new(ListNode {
        val: 0,
        next: None,
    });
    let mut tail = &mut new_list;
    while list1.is_some() && list2.is_some() {
        let (l1, l2) = (list1.as_mut().unwrap(), list2.as_mut().unwrap());
        if l1.val <= l2.val {
            let mut node = list1.take().unwrap();
            list1 = node.next.take();
            tail.next = Some(node);
        } else {
            let mut node = list2.take().unwrap();
            list2 = node.next.take();
            tail.next = Some(node);
        }
        tail = tail.next.as_mut().unwrap();
    }
    tail.next = if list1.is_some() { list1 } else { list2 };
    new_list.next
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut idx = 0;
    let mut i = 1;
    while i < nums.len() {
        if nums[i] != nums[idx] {
            idx += 1;
            nums[idx] = nums[i];
        }
        i += 1;
    }
    (idx + 1) as i32
}

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
    let mut empty_slots = Vec::<usize>::new();
    let mut count: usize = 0;
    while i < nums.len() {
        if nums[i] == val {
            empty_slots.insert(0, i);
        } else {
            count = count + 1;
            if let Some(empty_index) = empty_slots.pop() {
                nums[empty_index] = nums[i];
                empty_slots.insert(0, i);
            }
        }
        i = i + 1;
    }
    count as i32
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Two Sum Tests
    #[test]
    fn test_two_sum_basic() {
        let result = two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }

    // Is Palindrome Tests
    #[test]
    fn test_is_palindrome_true() {
        assert_eq!(true, is_palindrome(121));
    }

    #[test]
    fn test_is_palindrome_false() {
        assert_eq!(false, is_palindrome(12));
    }

    // Roman to Int Tests
    #[test]
    fn test_roman_to_int() {
        assert_eq!(1994, roman_to_int("MCMXCIV".to_string()));
    }

    // Longest Common Prefix Tests
    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(
            String::from("abcd"),
            longest_common_prefix(vec![String::from("abcd"), String::from("abcde")])
        );
    }

    // Is Valid Parentheses Tests
    #[test]
    fn test_is_valid_simple() {
        assert_eq!(true, is_valid("()".to_string()));
    }

    #[test]
    fn test_is_valid_multiple() {
        assert_eq!(true, is_valid("()[]{}".to_string()));
    }

    #[test]
    fn test_is_valid_invalid_pair() {
        assert_eq!(false, is_valid("(]".to_string()));
    }

    #[test]
    fn test_is_valid_wrong_order() {
        assert_eq!(false, is_valid("([)]".to_string()));
    }

    #[test]
    fn test_is_valid_nested() {
        assert_eq!(true, is_valid("{[]}".to_string()));
    }

    #[test]
    fn test_is_valid_unclosed() {
        assert_eq!(false, is_valid("([".to_string()));
    }

    #[test]
    fn test_is_valid_closing_only() {
        assert_eq!(false, is_valid("]".to_string()));
    }

    // Merge Two Lists Tests
    #[test]
    fn test_merge_two_lists_basic() {
        let l1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None,
                })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: None,
                })),
            })),
        }));
        let result = merge_two_lists(l1, l2);
        assert_eq!(listnode_to_vec(result), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn test_merge_two_lists_one_empty() {
        let l1 = None;
        let l2 = Some(Box::new(ListNode {
            val: 0,
            next: None,
        }));
        let result = merge_two_lists(l1, l2);
        assert_eq!(listnode_to_vec(result), vec![0]);
    }

    #[test]
    fn test_merge_two_lists_both_empty() {
        let l1 = None;
        let l2 = None;
        let result = merge_two_lists(l1, l2);
        assert_eq!(listnode_to_vec(result), vec![]);
    }

    // Remove Element Tests
    #[test]
    fn test_remove_element_basic() {
        let mut nums = vec![3, 2, 2, 3];
        let len = remove_element(&mut nums, 3);
        assert_eq!(len, 2);
        assert_eq!(&nums[0..len as usize], &[2, 2]);
    }

    #[test]
    fn test_remove_element_multiple() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let len = remove_element(&mut nums, 2);
        assert_eq!(len, 5);
        let mut result = nums[0..len as usize].to_vec();
        result.sort();
        assert_eq!(result, vec![0, 0, 1, 3, 4]);
    }

    #[test]
    fn test_remove_element_all_equal() {
        let mut nums = vec![2, 2, 2, 2];
        let len = remove_element(&mut nums, 2);
        assert_eq!(len, 0);
    }

    #[test]
    fn test_remove_element_none_equal() {
        let mut nums = vec![1, 2, 3, 4, 5];
        let len = remove_element(&mut nums, 6);
        assert_eq!(len, 5);
        assert_eq!(&nums[0..len as usize], &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_remove_element_empty() {
        let mut nums: Vec<i32> = vec![];
        let len = remove_element(&mut nums, 1);
        assert_eq!(len, 0);
    }

    #[test]
    fn test_remove_element_single_match() {
        let mut nums = vec![1];
        let len = remove_element(&mut nums, 1);
        assert_eq!(len, 0);
    }

    #[test]
    fn test_remove_element_single_no_match() {
        let mut nums = vec![1];
        let len = remove_element(&mut nums, 2);
        assert_eq!(len, 1);
        assert_eq!(&nums[0..len as usize], &[1]);
    }

    #[test]
    fn test_remove_element_multiple_positions() {
        let mut nums = vec![1, 1, 1, 2, 3, 1, 4, 1];
        let len = remove_element(&mut nums, 1);
        assert_eq!(len, 3);
        let mut result = nums[0..len as usize].to_vec();
        result.sort();
        assert_eq!(result, vec![2, 3, 4]);
    }
}
