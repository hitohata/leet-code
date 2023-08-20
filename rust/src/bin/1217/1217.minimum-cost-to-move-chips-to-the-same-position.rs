/*
 * @lc app=leetcode id=1217 lang=rust
 *
 * [1217] Minimum Cost to Move Chips to The Same Position
 */


// @lc code=start
impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {

        use std::collections::HashMap;

        let mut hash: HashMap<i32, usize> = HashMap::new();

        let mut cost = 0;

        position.iter().for_each(|el| {
            let cur = hash.get(el);
            match hash.get_mut(el) {
                Some(val) => {
                    *val += 1
                },
                None => {
                    hash.insert(el.to_owned(), 1);
                }
            }
        });

        fn target_is_odd(map: HashMap<i32, usize>) -> bool {
            let mut frec = 0;
            let mut max = 0;

            map.iter().for_each(|el| {
                // el.1 is frequency
                if el.1.to_owned() > frec {
                    // el.0 is number of position.
                    max = el.0.to_owned();
                }
            });

            if max % 2 == 0 {
                true
            } else {
                false
            }
        }

        let distinction_odd = target_is_odd(hash);

        position.iter().for_each(|el| {

            let target = el.to_owned();
            let target_odd = if target % 2 == 0 { true } else { false };

            if distinction_odd == target_odd {
                cost += 1;
            }
        });

        cost

    }
}
// @lc code=end
