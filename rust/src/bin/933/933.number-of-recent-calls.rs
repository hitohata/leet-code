/*
 * @lc app=leetcode id=933 lang=rust
 *
 * [933] Number of Recent Calls
 */

// @lc code=start
struct RecentCounter {
    called_time: std::collections::VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        RecentCounter {
            called_time: std::collections::VecDeque::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.called_time.push_back(t);

        for _ in 0..self.called_time.len() {
            if (self.called_time.front().unwrap() < &(t - 3000)) {
                self.called_time.pop_front();
            } else {
                break;
            }
        }

        self.called_time.len() as i32
    }
}


/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
// @lc code=end
