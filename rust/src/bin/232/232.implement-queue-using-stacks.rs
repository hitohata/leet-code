/*
 * @lc app=leetcode id=232 lang=rust
 *
 * [232] Implement Queue using Stacks
 */

// @lc code=start
struct MyQueue {
    front_stack: Vec<i32>,
    back_stack: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    fn new() -> Self {
        MyQueue {
            front_stack: vec![],
            back_stack: vec![]
        }
    }
    
    fn push(&mut self, x: i32) {
        self.front_stack.insert(0, x);
        self.back_stack.push(x);
    }
    
    fn pop(&mut self) -> i32 {
        self.back_stack.remove(0);
        self.front_stack.pop().unwrap()
    }
    
    fn peek(&self) -> i32 {
        self.back_stack.first().unwrap().to_owned()
    }
    
    fn empty(&self) -> bool {
        self.front_stack.is_empty()
    }
}
/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
// @lc code=end
