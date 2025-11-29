pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut write_idx = m as usize + n as usize - 1;
    let (mut i, mut j) = (m - 1, n - 1);

    while j >= 0 {
        if i >= 0 && nums1[i as usize] > nums2[j as usize] {
            nums1[write_idx] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[write_idx] = nums2[j as usize];
            j -= 1;
        }
        write_idx -= 1;
    }
}
mod tests {
    use super::*;
    #[test]
    fn test_merge_non_empty_arrays() {
        let (mut nums1, mut nums2) = (vec![1, 2, 3, 0, 0, 0], vec![2, 5, 6]);
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
}
