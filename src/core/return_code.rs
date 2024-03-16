/**
 * Return code enum.
 */
pub enum RC {
    /*
     * Return when the argument passed to the function is
     * invalid and the function should not assert.
     * Usage examples:
     * get_nth_word(string_ref : &String, n: usize) function
     * if there are less than n words in the string, return
     * an invalid argument.
     */
    CInvalidArgument,
}
/**
 * Note: when using
 * #[derive(Debug)]
 * pub enum RC{
 *     CInvalidArgument,
 * }
 * The compiler actually adds the next code:
 */
impl core::fmt::Debug for RC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            RC::CInvalidArgument => write!(f, "InvalidArgument"),
        }
    }
}
