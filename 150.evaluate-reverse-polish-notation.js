/*
 * @lc app=leetcode id=150 lang=javascript
 *
 * [150] Evaluate Reverse Polish Notation
 */

// @lc code=start
/**
 * @param {string[]} tokens
 * @return {number}
 */
var evalRPN = function(tokens) {
    const stack = [];

    for (let i = 0; i < tokens.length; i++) {
        if (isNaN(tokens[i])) {
            const num2 = Number(stack.pop());
            const num1 = Number(stack.pop());
            const res = calc(num1, num2, tokens[i]);
            stack.push(res);
        } else {
            stack.push(tokens[i])
        }
    };

    return stack[0]

};
// @lc code=end

/**
 * @param {number} num1
 * @param {number} num2
 * @param {string} operator
 * @return {number}
 */
const calc = (num1, num2, operator) => { 
    if (operator === "+") return num1 + num2;
    if (operator === "-") return num1 - num2; 
    if (operator === "*") return num1 * num2;
    if (operator === "/") return Math.trunc(num1 / num2);
}
