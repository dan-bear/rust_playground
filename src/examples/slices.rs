pub struct PgSlices;

impl PgSlices {
    pub fn example() {
        println!("SLICES");
        PgSlices::basics();
        PgSlices::range_syntax();
        PgSlices::slice_reference_ownership();
        PgSlices::get_first_word_example();
        PgSlices::string_literals();
        PgSlices::string_slice_deref_coercion();
        PgSlices::arr_slices();
        crate::core::utils::Utils::print_line_separator();
    }

    fn basics() {
        let example_str = String::from("pi is not that tasty");
        /* The last index is exclusive! */
        let pi_slice: &str = &example_str[0..2];    
        let is_slice: &str = &example_str[3..5];
        let not_slice: &str = &example_str[6..9];
        let example_str_full_slice: &str = &example_str;
        /*
         * The slices allow to reference parts of the string.
         * Note that the example_str is immutable, thus all the
         * slices can co-exist and used together since they are
         * all immutable (read-only).
         */
        println!(
            "\"{pi_slice} {is_slice} {not_slice}\" is a \
         prefix of \"{example_str_full_slice}\""
        );
        /*
         * Slices are fat-pointers, i.e. a pointer and the
         * number of bytes followed from the pointed memory
         * address which are represented by the slice.
         */
    }

    fn range_syntax() {
        let example_str = String::from("life is good");
        let slice_it_all: &str = &example_str[..];
        let first_word_slice1: &str = &example_str[0..4];
        let first_word_slice2: &str = &example_str[..4];
        let last_word_slice1: &str = &example_str[8..12];
        let last_word_slice2: &str = &example_str[8..];
        println!("{slice_it_all}");
        println!("{first_word_slice1}");
        println!("{first_word_slice2}");
        println!("{last_word_slice1}");
        println!("{last_word_slice2}");
    }

    fn slice_reference_ownership() {
        /*
         * Slices like references borrows the memory.
         * The next example does the follows:
         * 1: define a mutable string (mutable memory on heap).
         * 2: define a slice, which borrows the ownership of
         *    the mutable string (It can read the data).
         * 3: When trying to change the mutable string the
         *    compiler does not allow it since the memory
         *    is borrowed by the slice. (The compiler does not
         *    allow to WRITE to memory which can be READ by
         *    another variable).
         */
        let mut example_str = String::from("pi is not that tasty");
        let _pi: &str = &example_str[0..2]; /* <= immutable borrow */
        example_str.push_str("3.1415926535"); /* <= mutable borrow*/
        /* Uncomment to see the compiler error. */
        //println!("first word is: {_pi}");
        /*
         * Note: without the last read (println!("first word is: {pi}),
         * the compiler can determine there is not going to be
         * any other read-access to the slice (reference) and
         * the code would compile.
         */

        /*
         * Another similar example.
         */
        let mut example_str = String::from("life is good");
        let _last_word: &str = &example_str[8..]; /*<= immutable borrow.*/
        example_str.clear(); /* <= mutable borrow */
        /*
         * If the next line is uncommented, the compiler
         * rises an error, last_word is an immutable borrow
         * of example_str and the clear() method is a mutable
         * borrow of example_str, as it changes its data.
         */
        //println!("Last word is: {_last_word}");
    }

    fn get_first_word_example() {
        let str_example = String::from("life is good");
        let first_word: &str = PgSlices::get_first_word(&str_example);
        /*
         * Ownership was not lost since the str_example was
         * passed by reference.
         */
        println!("String is: {str_example}");
        println!("First word is: {first_word}");
    }

    /**
     * The method gets the first word of a string.
     * @param string_ref an immutable reference to a string.
     * @return a slice-reference to the first word in the string.
     */
    fn get_first_word(string_ref: &String) -> &str {
        let space: u8 = b' ';
        let byte_arr = string_ref.as_bytes();
        for (char_idx, &char) in byte_arr.iter().enumerate() {
            if char == space {
                return &string_ref[0..char_idx];
            }
        }
        return &string_ref[0..string_ref.len()];
    }

    fn string_literals() {
        /*
         * Reminder: string literals are stored in the
         * binary file, unlike the String type whose memory
         * is allocated in the heap. String literals must
         * be immutable, as the binary file cannot
         * (maybe should not) be accessed for write during
         * run time.
         */
        let str_literal = "life is good";
        /*
         * Actually, the string literal type is a string
         * slice. When thinking about it, it should be
         * pretty reasonable, since "life is good" bytes
         * are stored in the binary file, so to access them
         * a pointer and size is needed, string slices are
         * just coupling of pointer and size.
         * So, the size of str_literal variable is:
         * pointer_size_in_bytes + length_size_in_bytes,
         * so it's at most 16 bytes (as the maximal length
         * is at most 2^64 - 1, so 8 bytes are enough).
         */
        println!(
            "The string literal is: {str_literal} and size \
            of string slice is {} bytes",
            std::mem::size_of::<&str>()
        );
    }

    fn string_slice_deref_coercion() {
        /*
         * Due to the IMPLICIT DEREF COERSIONS WITH
         * FUNCTIONS AND METHODS
         * When passing a String reference (&String) re
         *  (will not be address here).
         * Passing reference to a String (&String) and
         * String-slice (&str) as a parameter to a function
         * is the same thing. Thus, to allow broader usage
         * of methods and functions, passig
         */

        let str_example = String::from("Life is good");
        let p_second_word: &str = &str_example[5..7];
        /* Explicit usage */
        PgSlices::print_string_reference(&str_example);
        PgSlices::print_string_slice(p_second_word);

        /*
         * &String variable does not couple ptr and length,
         * but just the ptr, let's validate it using the
         * size_of method.
         */
        println!(
            "String ref (&String) size in bytes is: {} \n\
            String slice (&str) size in bytes is {}",
            std::mem::size_of::<&String>(),
            std::mem::size_of::<&str>()
        );

        /*
         * The size difference makes it impossible for a
         * function expecting a &String as a parameter to
         * get a &str as a param.
         * Does not compile:
         */
        //PgSlices::printStringReference(p_second_word);
        /*
         * But the other way around works due to deref
         * coercion (It's also possible to automatically
         * add the data needed to cast a &String to &str).
         */
        PgSlices::print_string_slice(&str_example);
    }

    fn print_string_reference(string_ref: &String) {
        println!("The string is: {string_ref}");
    }

    fn print_string_slice(string_slice: &str) {
        println!("The string is: {string_slice}");
    }

    fn arr_slices() {
        let arr: [u64; 5] = [2, 3, 5, 7, 11];
        let arr_slice: &[u64] = &arr[0..3];
        /* The array slice type is &[data_type] */
        println!("The size of array slice is {}", std::mem::size_of::<&u64>());
        println!("The array is {:?} and the slice is {:?}", arr, arr_slice);
    }
}
