pub struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0 as usize;
        let mut right = nums.len() - 1;
        if target <= nums[left] {
            return 0;
        }
        if target == nums[right] {
            return right as i32;
        }
        if target > nums[right] {
            return right as i32 + 1;
        }
        while (left <= right) {
            let mid = left + (right - left) / 2;
            if target == nums[mid] {
                return mid as i32;
            } else if target > nums[mid] && target < nums[mid + 1] {
                return (mid + 1) as i32;
            } else if target < nums[mid] && target > nums[mid - 1] {
                return mid as i32;
            } else if target > nums[mid] {
                left = mid + 1;
            } else if target < nums[mid] {
                right = mid - 1;
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert_1() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    }
    #[test]
    fn test_search_insert_2() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    }
    #[test]
    fn test_search_insert_3() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }
    #[test]
    fn test_search_insert_4() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    }
    #[test]
    fn test_search_insert_5() {
        assert_eq!(Solution::search_insert(vec![1], 0), 0);
    }
}
