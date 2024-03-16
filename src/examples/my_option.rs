pub enum MyOption<T> {
    None,
    Some(T),
}

/**
 * Rhe deref operator:
 *
 * The deref operator is implemented by implementing
 * the deref trait:
 *
 * pub trait Deref {
 *   type Target: ?Sized;
 *
 *   // Required method
 *   fn deref(&self) -> &Self::Target;
 * }
 *
 * Let's have a look in the function signature:
 * fn deref(&self) -> &Self::Target
 * - The input the a deref is a reference of an object,
 *   it's pretty logical, as the object of dereferencing
 *   is some object.
 * - The output is a reference to Self::Target:
 *   - "Self::Target" some value needs to be returned from
 *      the operator, however, it is not known in advance
 *      what's the data which is about to be dereferenced.
 *      Thus, the type Target: ?Sized is needed. Type is
 *      used for type aliasing, when used in a trait it
 *      declares an associated type. In the trait context,
 *      some type needs to be returned from it, thus it
 *      defines an associated type. When implementing the
 *      Deref trait the associated type must be explicitly
 *      defined, otherwise, the return value from deref is
 *      not defined.
 *      - Note - the deref operator may return a different
 *        type than the referenced type, also it not the
 *        common thing to do, Target can be any type.
 *    - The "?Sized" syntax:
 *        - Sized is used to declare a bound on a type. It
 *          is needed, since storing values in stack or
 *          passing them to a function is needed.
 *        - It's possible that the dereferenced type does
 *          not have a size bound, thus the ?Sized is used,
 *          so one can implement the Deref trait for an
 *          unbounded size type.
 *    - Why deref returns a reference?
 *      
 *
 *
 * - The "type Target: ?Sized" syntax:
 *   1: The type keyword is used for aliasing, when used in
 *      a trait, like in the Deref example, it is used to
 *      declare an associated type. Let's look in the
 *
 *   2:  There is an implicit Sized bound on associated
 *       types that can be relaxed using the special
 *       ?Sized bound.
 *
 * pub trait der
 *
 *
 *
 * The deref operator (* operator) is used to get a
 * value out of a
 *
 *
 *
 */

impl<T> MyOption<T> {
    /**
     * Takes does the followings:
     * 1: Create a copy of the option object.
     * 2: Saves None in the current option object.
     *
     * Note: the std::mem::replace does memory swap in an
     * unsafe block.
     */
    fn take(&mut self) -> MyOption<T> {
        return std::mem::replace(self, MyOption::None);
    }
}

/**
 * Implementing deref() for MyOption.
 */
// impl<T> std::ops::Deref for MyOption<T> {
//     /*
//      * In case MyOption is None, a None is returned.
//      */
//     type Target = T;

//     fn deref(&self) -> &T {
//         match &self {
//             MyOption::None => &MyOption::None,
//             MyOption::Some(val) => &val,
//         }
//     }
// }

pub struct MyOptionExample;

impl MyOptionExample {
    pub fn run_example() {
        //MyOptionExample::deref_my_option();
        MyOptionExample::take_example();
        crate::core::utils::Utils::print_line_separator();
    }

    fn take_example() {
        let val: u64 = 2;
        let mut my_option: MyOption<u64> = MyOption::Some(val);
        let taking_my_option: MyOption<u64> = my_option.take();
        match my_option {
            MyOption::Some(_x) => println!("Bug in take() impl"),
            MyOption::None => println!("Cool, take() works!"),
        }
        match taking_my_option {
            MyOption::Some(x) => {
                assert!(x == val);
                println!("The taken value is {}", x);
            }
            MyOption::None => println!("Bug in take() impl"),
        }
    }

    // fn deref_my_option() {
    //     // let maybe_val1: MyOption<u64> = MyOption::None;
    //     // let maybe_val2: MyOption<u64> = MyOption::Some(2);
    //     // let deref_maybe_val1 = *maybe_val1;
    //     // let deref_maybe_val2 = *maybe_val2;
    //     // assert!(deref_maybe_val1 == MyOption::None);
    //     //     assert!(deref_maybe_val2 == 2);
    // }
}
