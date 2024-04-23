/*
 * @lc app=leetcode id=705 lang=javascript
 *
 * [705] Design HashSet
 */

// @lc code=start

var MyHashSet = function() {
    set = [];
};

/** 
 * @param {number} key
 * @return {void}
 */
MyHashSet.prototype.add = function(key) {
    set = [...set.filter(el => el !== key), key]
};

/** 
 * @param {number} key
 * @return {void}
 */
MyHashSet.prototype.remove = function(key) {
    set = set.filter(el => el !== key)
};

/** 
 * @param {number} key
 * @return {boolean}
 */
MyHashSet.prototype.contains = function(key) {
    set = set.filter(el => el !== key)
};

/** 
 * Your MyHashSet object will be instantiated and called as such:
 * var obj = new MyHashSet()
 * obj.add(key)
 * obj.remove(key)
 * var param_3 = obj.contains(key)
 */
// @lc code=end
