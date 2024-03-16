use std::num::ParseIntError;

pub struct PgTurboFish;

impl PgTurboFish {
    pub fn run_example() {
        /*
         * The turbo-fish feature allows to help the
         * compiler to infer the type for generic types
         * (structs).
         */

        /*
         * The next line does not compile since the compiler
         * cannot infer the type of x.
         * parse() signature is:
         * pub fn parse<F>(&self) -> Result<F, F::Err>
         * The compiler error is:
         * "consider giving `x` an explicit type, where the
         * type for type parameter `F` is specified:
         * `: Result<F, _>`".
         */
        //let x = "31415926535".parse();

        /*
         * To fix it, one can define the variable type
         * explicitly.
         */
        //let res0: u64 = "31415926535".parse();

        /*
         * This does not work since parse returns result
         * and not a u64.
         * One way to solve it, is to use the unwrap method
         * of Result<T,Err>.
         *
         * Note - in case an error is returned and
         * Result::unwrap() is used the system panics.
         */
        let _u64_pi: u64 = "31415826535".parse().unwrap();

        /*
         * Another way is to define explicitly the result
         * type, this way, if parse fails the code can
         * try and handle it.
         */
        let _res_u64_pi: Result<u64, ParseIntError> =
            "31415926535".parse();
        /*
         * The drawback* (see note below) of this way is
         * that one needs to be familiar with the parse
         * Result::Err types, which need to correspond to
         * the returned value type. For example,
         * ParseBoolError or ParseCharError.
         *
         * Note - knowing the return value types of a func
         * is pretty important, so one coul d say that
         * declaring the Result<T, T::Err> explicitly is
         * actually the preferable way and not a drawback
         * (it makes the coder to know exactly what is
         * going on).
         */

        /*
         * The turbofush syntax allows to tell the compiler
         * what's the wished return type without declaring
         * the Result<T, T::Err> explicitly when the
         * variable storing the result is initialized:
         */
        let _turbofish_res_u64_pi = "31415926535".parse::<u64>();

        crate::core::utils::Utils::print_line_separator();
    }
}
