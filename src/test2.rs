pub struct Solution {
    bad: i32,
}

impl Solution {
    fn new(bad: i32) -> Solution {
        Solution { bad }
    }
    pub fn isBadVersion(&self, version: i32) -> bool {
        if version < self.bad {
            return false;
        } else {
            return true;
        }
    }

    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut below = 1;
        let mut above = n;
        if self.isBadVersion(below) {
            return below;
        }
        let mut last_false_below = below;
        // ffftttt
        loop {
            let below_rs = self.isBadVersion(below);
            let after_below_rs = self.isBadVersion(below + 1);
            if !below_rs && after_below_rs {
                return below + 1;
            } else if below_rs && after_below_rs {
                above = below;
            } else if !below_rs {
                last_false_below = below;
            }
            below = last_false_below + (above - last_false_below) / 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_search_1() {
        let n = 500;
        let bad = 400;
        let s = Solution::new(bad);
        assert_eq!(s.first_bad_version(n), bad);
    }

    #[test]
    #[ignore]
    fn test_search_2() {
        let n = 2126753390;
        let bad = 1702766719;
        let s = Solution::new(bad);
        assert_eq!(s.first_bad_version(n), bad);
    }
}
