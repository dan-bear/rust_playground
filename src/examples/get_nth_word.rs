/** Shortcut to RC */
use crate::core::return_code::RC;

//_____________________________________________________________________________
pub struct PgGetNthWord;
impl PgGetNthWord {
    /**
     * The public example method, can be called by the
     * outside word.
     */
    pub fn example() {
        let sentence = String::from("life is good");
        let first_word_num: usize = 1;
        let forth_word_num: usize = 4;

        let first_word_or_rc: Result<
            &str,
            crate::core::return_code::RC,
        > = PgGetNthWord::get_nth_word(&sentence, first_word_num);
        PgGetNthWord::display_result(
            first_word_or_rc,
            first_word_num,
        );

        let forth_word_or_rc: Result<&str, RC> =
            PgGetNthWord::get_nth_word(&sentence, forth_word_num);
        PgGetNthWord::display_result(
            forth_word_or_rc,
            forth_word_num,
        );
        crate::core::utils::Utils::print_line_separator();
    }

    /*Private methods */

    /**
     * Get's the nth word in a string.
     * @param string_ref a reference to a string / a string
     * slice. (deref implicit coercion).
     * @param n the word number to get. It's asserted the value
     * is greater than 0.
     * @return a slice-reference to the nth word if exists.
     * @return RC::CInvalidArgument in case there are less than
     * n words in the string.
     * @note an empty word might be returned, since the
     * separator is a space. For example " life is good", the
     * first word is the empty word since the string starts with
     * a space.
     */
    fn get_nth_word(string_ref: &str, n: usize) -> Result<&str, RC> {
        assert!(n > 0);
        let space_int_val: u8 = b' ';
        let mut space_counter: usize = 0;
        /*An index has not been found yet*/
        let mut first_char_idx: usize = 0;
        let byte_arr = string_ref.as_bytes();
        for (char_idx, &char) in byte_arr.iter().enumerate() {
            if char == space_int_val {
                space_counter += 1;
                if space_counter == n {
                    return Ok(&string_ref[first_char_idx..char_idx]);
                }
                /* The next word starts in char_idx + 1 */
                first_char_idx = char_idx + 1;
            }
        }
        /*
         * Two possibilities:
         * 1: No space was found.
         * 2: The is no nth word.
         */
        if space_counter == 0 && n == 1 {
            return Ok(&string_ref[0..string_ref.len()]);
        }
        return Err(RC::CInvalidArgument);
    }

    /**
     * Displays the results of the get_nth_word method.
     * @param result the result returned from the
     * get_nth_word method.
     */
    fn display_result(result: Result<&str, RC>, word_num: usize) {
        match result {
            Ok(nth_word) => {
                println!("The {word_num}th word is: {nth_word}")
            }
            /*
             * Note: RC implements std::fmt::Debug and doe not
             * implement std::fmt::Display. That's why {:?} is
             * used to print the rc value.
             */
            Err(rc) => println!("The return code is {:?}", rc),
        }
    }
}
