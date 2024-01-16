/*
 * @lc app=leetcode id=933 lang=rust
 *
 * [933] Number of Recent Calls
 */

// @lc code=start
struct RecentCounter {
    called_time: Vec<i32>,
    pin: usize
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        RecentCounter {
            called_time: vec![],
            pin: 0
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.called_time.push(t);

        let smallest = t - 3000;
        let largest = t;
        let mut count = 0;

        for i in (self.pin..(self.called_time.len())).rev() {
            if smallest <= self.called_time[i] && self.called_time[i] <= largest {
                count += 1;
                self.pin = i;
            } else {
                break;
            }
        }

        count
    }
}


/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
// @lc code=end
