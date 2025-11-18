fn main() {
    let result = two_sum(vec![2, 7, 11, 15], 9);
    assert_eq!(result, vec![0, 1]);

    assert_eq!(true, is_palindrome(121));
    assert_eq!(false, is_palindrome(12));

    assert_eq!(1994, roman_to_int("MCMXCIV".to_string()));

    assert_eq!(
        String::from("abcd"),
        longest_common_prefix(vec![String::from("abcd"), String::from("abcde")])
    );

    assert_eq!(true, is_valid("()".to_string()));
    assert_eq!(true, is_valid("()[]{}".to_string()));
    assert_eq!(false, is_valid("(]".to_string()));
    assert_eq!(false, is_valid("([)]".to_string()));
    assert_eq!(true, is_valid("{[]}".to_string()));
    assert_eq!(false, is_valid("([".to_string()));
    assert_eq!(false, is_valid("]".to_string()));
    assert_eq!(false, is_valid("([".to_string()));

    // Tests for merge_two_lists
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

    // Convert result to Vec for easy comparison
    fn listnode_to_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = vec![];
        while let Some(node) = list {
            v.push(node.val);
            list = node.next;
        }
        v
    }
    assert_eq!(listnode_to_vec(result), vec![1,1,2,3,4,4]);

    // Test with one empty list
    let l1 = None;
    let l2 = Some(Box::new(ListNode {
        val: 0,
        next: None,
    }));
    let result = merge_two_lists(l1, l2);
    assert_eq!(listnode_to_vec(result), vec![0]);

    // Test with both empty lists
    let l1 = None;
    let l2 = None;
    let result = merge_two_lists(l1, l2);
    assert_eq!(listnode_to_vec(result), vec![]);
    
}

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
        // first and second are &[char], so destructure by indexing
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
