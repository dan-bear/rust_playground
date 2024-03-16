/*
 * https://leetcode.com/problems/add-two-numbers/
 */

// /*Definition for singly-linked list.*/

// #[derive(PartialEq, Eq, Clone, Debug)]
// /*
// * Note: I would recommend to define getters and setters
// * and define the members as private, but that's the leet
// * code definition.
// */
// pub struct ListNode {
//     /**means it can be none or a pointer to ListNode */
//     pub next: Option<Box<ListNode>>,
//     pub val: i32,
// }

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

// /**
//  * The list head.
//  */
// pub struct ListHead<'a> {
//     pub m_next: &'a Option<Box<ListNode>>,
// }

// /**
//  * Reminder:
//  * Option<T>: optional value, either "Some" T or "None".
//  * Box<T>: T is allocated on the heap by value/ Box is a
//  *         smart pointer, when a box goes out of scope its
//  *         destructor is called and the inner object is
//  *         destroyed.
//  */

// /**
//  * For tomorrow - create a linked list and iterate a linked
//  * list (with optional).
//  */

// struct Solution;

// impl Solution {
//     pub fn add_two_numbers(
//         l1: Option<Box<ListNode>>,
//         l2: Option<Box<ListNode>>,
//     ) -> Option<Box<ListNode>> {
//         /* What's the idiomatic way of checking for Nones? */
//         if l1 == None && l2 == None {
//             return None;
//         }
//         let res: Option<Box<ListNode>> = Some(Box::new(ListNode::new(2)));
//         return res;

//         // if l1 == None && l2 == None {
//         //     return None;
//         // }

//         // let mut current_node_1 = l1;
//         // let mut current_node_2 = l2;

//         // let mut current_sum: i32 = 0;
//         // let mut current_val: i32 = 0;
//         // let mut current_carry: i32 = 0;

//         // let mut res: Option<Box<ListNode>> =
//         //     Some(Box::new(ListNode::new(0)));

//         // let mut next_res_node = res.unwrap().next;

//         // while current_node_1 != None && current_node_2 != None {
//         //     /*
//         //      * Use unwrap as it checked the nodes are not null.
//         //      */
//         //     current_sum = current_carry
//         //         + current_node_1.unwrap().val
//         //         + current_node_2.unwrap().val;

//         //     current_val = current_sum % 10;
//         //     current_carry = (current_sum - current_val) / 10;

//         //     /* Update the */
//         //     current_res_node.unwrap().val = current_val;
//         // }

//         // return res;
//         // // while current_list_node_1 != None && current_list_node_2 != None {
//         // //     let mut sum: i32 = current_carry
//         // //         + current_list_node_1.val
//         // //         + current_list_node_2.val;
//         // //     current_val = sum % 10;
//         // //     current_carry = (sum - current_val) / 10;

//         // //     /* Update res */
//         // //     res.val = current_val;
//         // //     /* Create a new node. */

//         // //     /* Make a new node and move res to point it. */

//         // //     /* Move current_list_node1 and current_list node2 */
//         // // }
//     }
// }

// pub struct TestSolution;

// impl TestSolution {
//     pub fn test_solution() {
//         //Solution::create_and_iterate_list();
//         // let mut num1: Option<Box<ListNode>> =
//         //     Some(Box::new(ListNode::new(2)));
//         // let mut num2: Option<Box<ListNode>> =
//         //     Some(Box::new(ListNode::new(3)));
//         // let sol = Solution::add_two_numbers(num1, num2);
//     }
// }
