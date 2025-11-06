fn main() {
    let result = two_sum(vec![2, 7, 11, 15], 9);
    assert_eq!(result, vec![0, 1]);

    assert_eq!(true, is_palindrome(121));
    assert_eq!(false, is_palindrome(12));
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
