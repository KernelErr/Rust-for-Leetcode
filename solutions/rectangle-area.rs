use std::cmp::{max, min};
impl Solution {
    pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
        let mut ans: i32 = (ay2 - ay1) * (ax2 - ax1) + (by2 - by1) * (bx2 - bx1);
        let w = min(ax2, bx2) - max(ax1, bx1);
        let h = min(ay2, by2) - max(ay1, by1);
        if w < 0 || h < 0 {
            ans
        } else {
            ans - w * h
        }
    }
}