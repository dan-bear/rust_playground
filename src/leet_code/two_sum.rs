/**
 * https://leetcode.com/problems/two-sum/
 *
 *
 */

pub struct Solution; //When submitting, remove this line.

enum RC {
    CNotFound,
    CNoSolution,
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let res: Result<Vec<i32>, RC> =
            Solution::two_sum_square_time_complexity(&nums, target);
        match res {
            Ok(two_sum_vec) => return two_sum_vec,
            Err(_rc) => return Vec::from([-1 as i32, -1 as i32]),
        }
    }

    /**
     * A Vec.len^2 time complexity solution.
     * @param nums the vector passed as a slice.
     * @param target the value whose two-sum need to be found.
     * @return if the target two-sum solution is found, a
     * vector containing two indexes which representing a
     * solution. Otherwise, a vector with two invalid
     * index is returned (Vec[-1, -1]).
     */
    fn two_sum_square_time_complexity(
        nums: &[i32],
        target: i32,
    ) -> Result<Vec<i32>, RC> {
        for (idx, &elem) in nums.iter().enumerate() {
            if let Ok(complement_idx) =
                Solution::search_complement(&nums, idx, target - elem)
            {
                let res_vec: Vec<i32> =
                    Vec::from([idx as i32, complement_idx as i32]);
                return Ok(res_vec);
            }
            /* Move to the next element */
        }
        /* No solution exists, so invalid indexes are returned. */
        return Err(RC::CNoSolution);
    }

    /**
     * @param nums - a i32 slice.
     * @param forbidden_idx - the index which should not be
     * used to find the target.
     * @param target - the value to find in the (Vec) slice.
     */
    fn search_complement(
        nums: &[i32],
        forbidden_idx: usize,
        target: i32,
    ) -> Result<usize, RC> {
        for (idx, &elem) in nums.iter().enumerate() {
            if idx != forbidden_idx && elem == target {
                return Ok(idx);
            }
        }
        return Err(RC::CNotFound);
    }
}
