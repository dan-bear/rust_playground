/*
 * How match works:
 *
 */

pub struct MatchExample;

impl MatchExample {
    pub fn run_example() {
        MatchExample::dereference_input();
        crate::core::utils::Utils::print_line_separator();
    }
}

/*
* Matching an input can borrow input substructure,
* without taking ownership. It is needed to match a
* reference.
*
* the input to match is an L-value expression, this means
* that the input expression is evaluated to a memory
* location where the value lives. Match works by doing this
* evaluation and then inspecting the data at that memory
* location.
*
* Thus, when matching a reference, the dereference operator
* can be used (in the past it was needed), when using match
* as the dereferences allows the match to inspect the memory
* location of the variable when a reference to the variable
* is passed.
*/
impl MatchExample {
    fn dereference_input() {
        let str1: Option<String> = Option::Some(String::from("A"));
        let str2: Option<String> = Option::None;
        MatchExample::print_optional_str(&str1);
        MatchExample::print_optional_str(&str2);
    }

    fn print_optional_str(r_opt_str: &Option<String>) {
        /*
         * Dereferece the reference so the data can be
         * inspected by match.
         */
        match *r_opt_str {
            /*
             * ref is used, otherwise  the println! takes
             * the value of the string which was passed by
             * reference. Rust diagnoses that the value
             * exists in another scope (which passed the
             * reference) so it cannot move the value to
             * println! as it belongs to the outer scope.
             * The ref key-word tells to get a reference to
             * the value represented in the Option's Some
             * variant
             */
            Option::Some(ref r_str) => println!("The string is {}", r_str),
            Option::None => println!("There is no string"),
        }
    }
}

//read about match an ownership.
//https://blog.rust-lang.org/2015/04/17/Enums-match-mutation-and-moves.html
