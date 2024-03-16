pub struct BubbleSort;

//TODO why it does not recognize the other method without
//the struct name?

//TODO: Where the memory of the array is saved at?

//TODO: Explain why the next syntax cannot work:
//let arr_of_arr: [mut [u64; 3]; 6].

impl BubbleSort {
    pub fn example() {
        let arr_of_arr: [&mut [u64; 3]; 6] = [
            &mut [2, 3, 5],
            &mut [2, 5, 3],
            &mut [3, 2, 5],
            &mut [3, 5, 2],
            &mut [5, 2, 3],
            &mut [5, 3, 2],
        ];

        for arr in arr_of_arr {
            println!("Unsorted array is: {:?}", arr);
            BubbleSort::u64_arr_bubble_sort(arr);
            println!("Sorted array is: {:?}", arr);
        }

        crate::core::utils::Utils::print_line_separator();
    }

    /**
     * @param arr_slice a mutable slice of an array, so it
     * can be swapped.
     */
    fn u64_arr_bubble_sort(arr_slice: &mut [u64]) {
        let num_of_elements: usize = arr_slice.len();
        for _elem_idx in 0..num_of_elements {
            BubbleSort::u64_run_bubble_step(arr_slice);
        }
    }

    fn u64_run_bubble_step(arr_slice: &mut [u64]) {
        let elem_num: usize = arr_slice.len();
        let mut idx: usize = 0;
        while idx < elem_num - 1 {
            if arr_slice[idx] > arr_slice[idx + 1] {
                BubbleSort::u64_swap_with_successor(arr_slice, idx);
            }
            idx += 1;
        }
    }

    fn u64_swap_with_successor(
        arr_slice: &mut [u64],
        elem_idx: usize,
    ) {
        /* Validate memory access */
        assert!(elem_idx < arr_slice.len() - 1);
        let temp: u64 = arr_slice[elem_idx];
        arr_slice[elem_idx] = arr_slice[elem_idx + 1];
        arr_slice[elem_idx + 1] = temp;
    }
}
