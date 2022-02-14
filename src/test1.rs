pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut below_index = 0;
    let mut above_index = nums.len() - 1;
    while below_index <= above_index {
        let mid = (below_index + above_index) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if target < nums[mid] {
            if mid == 0 {
                return -1;
            } else {
                above_index = mid - 1;
            }
        } else if target > nums[mid] {
            below_index = mid + 1;
        }
    }
    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_search() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 3), 2);
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
        assert_eq!(search(vec![2, 5], 5), 1);
        assert_eq!(search(vec![-1, 0, 5], -1), 0);
        assert_eq!(search(vec![-1, 0, 5], 5), 2);
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
        assert_eq!(search(vec![5], -5), -1);
    }
}
