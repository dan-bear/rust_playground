/**
 * Managing projects:
 *
 *
 *
 * 4: Modules cheat sheet:
 *   4.1: Start from the crate root:
 *        When compiling a crate, the compiler first looks
 *        in the crate root file (usually src/lib.rs for a
 *        library crate or src/main.rs for a binary crate)
 *        for code to compile.
 *   4.2: Declaring modules:
 *        In the crate root file, you can declare new
 *        modules; say, you declare a “garden” module with
 *        "mod garden;" or "mod garden {impl...}" syntax.
 *        The compiler will look for the module’s code in
 *        these places:
 *        - Inline: within curly brackets that replace the
 *           semicolon following mod garden (if the
 *           "mod garden { impl... } syntax was used").
 *        - In the file src/garden.rs
 *        - In the file src/garden/mod.rs
 *   4.3; Declaring submodules:
 *        In any file other than the crate root, you can
 *        declare submodules. For example, you might declare
 *        mod vegetables; in src/garden.rs. The compiler
 *        will look for the submodule’s code within the
 *        directory named for the parent module in these
 *        places:
 *        - Inline, directly following mod vegetables,
 *          within curly brackets instead of the semicolon
 *        - In the file src/garden/vegetables.rs
 *        - In the file src/garden/vegetables/mod.rs
 *   4.4: Paths to code in modules:
 *        Once a module is part of your crate, you can
 *        refer to code in that module from anywhere else
 *        in that same crate, as long as the privacy rules
 *        allow, using the path to the code. For example,
 *        an Asparagus type in the garden vegetables module
 *        would be found at:
 *        crate::garden::vegetables::Asparagus.
 *   4.5: Private vs public:
 *        Code within a module is private from its parent
 *        modules by default. To make a module public,
 *        declare it with pub mod instead of mod.
 *        To make items within a public module public
 *        as well, use pub before their declarations.
 *   4.6: The use keyword:
 *        Within a scope, the use keyword creates shortcuts
 *        to items to reduce repetition of long paths.
 *        In any scope that can refer to
 *        crate::garden::vegetables::Asparagus, you can
 *        create a shortcut with
 *        use crate::garden::vegetables::Asparagus;
 *        and from then on you only need to write Asparagus
 *        to make use of that type in the scope.
 *
 *   
 */

/**
 * All the modules in the core directory are sub-modules to
 * core. This syntax is the inline way to do it.
 * Note: using this syntax there is no need to have a
 * core.rs file in the core directory.
 */
// mod core{
//     pub mod return_code;
// }
/**
* I personally does not like to use the "inline sub-module"
* syntax (For now, 230803 it's just a matter of taste and
* my cpp background). So, another option (which I am going
* take) is:
* 1: Declare the core module in main.rs.
* 2: Add src/core.rs file.
* 3: Add the raw "pub mod return_code" in src/core/core.rs.
*
*
*




*/

/**
 * 230903
 *
 *
 * File structure:
 *
 * Option1 - under src directory
 * main.rs
 * lib.rs
 * dir1:
 *    - mod.rs
 *    - module1_1.rs
 *    - module1_2.rs
 *    ...
 *    - module1_n.rs
 * dir2:
 *    - mod.rs
 *    - module2_1.rs
 *    - module2_2.rs
 *    ...
 *    - module2_n.rs
 * dir3:
 *    - mod.rs
 *    - module3_1.rs
 *    - module3_2.rs
 *    ...
 *    - module3_n.rs
 *    - sub_dir1:
 *        - mod.rs
 *        - mod3_sub_1_1.rs
 *        - mod3_sub_1_2.rs
 *        ...
 *        - mod2_sub_1_n.rs
 *    - sub_dir2:
 *        - mod.rs
 *        - mod3_sub_2_1.rs
 *        - mod3_sub_2_2.rs
 *        ...
 *        - mod2_sub_2_n.rs      
 *
 * In this option, lib.rs should contain:
 * pub mod dir1;
 * pub mod dir2;
 * pub mod dir3;
 * i.e. all the names of src's child directories.
 *
 * Each src/dir<num>/mod.rs file should contain:
 * pub mod <file_name_1>;
 * pub mod <file_name_2>;
 * ...
 * pub mod <file_name_n>;
 * for each .rs file in the directory.
 * And:
 * pub mod <sub_directory_name_1>;
 * pub mod <sub_directory_name_2>;
 * ...
 * pub mod <sub_directory_name_k>;
 * for each sub directory of src/dir<num>.
 *  
 *
 * to use definitions (function, structs, enums and etc..)
 * from src/main.rs:
 * <project_name>::<full_dir_path_from_src_with_::_as_separation_symbol>::definition_name
 * from other files:
 * crate::<full_dir_path_from_src_with_::_as_separation_symbol>::definition_name
 *
 *
 * Option2:
 * main.rs
 * dir_1.rs
 * dir_2.rs
 * ...
 * dir_n.rs
 * dir1:
 *  - mod_1_1.rs
 *  - ...
 *  - mod_1_k.rs
 * ...
 * dir_n:
 *  - mod_n_1.rs
 *  - ...
 *  - mod_n_k.rs
 *
 * when the dir_x.rs file contains the lines: pub mod mod_x_y;
 * for every mod_x_y.rs file in the dir_x directory.
 *
 *
 *
 */

fn main() {
    rust_pg::leet_code::calc_num_len::run_example();
    rust_pg::leet_code::get_max_digit::run_example();

    // rust_pg::examples::first_steps::FirstSteps::run_example();
    // rust_pg::examples::ownership::pg_ownership();
    // rust_pg::examples::slices::PgSlices::example();
    // rust_pg::examples::pointers::pg_pointers();
    // rust_pg::examples::unsafe_func::pg_unsafe_functions();
    // rust_pg::examples::first_word_len::pg_first_word_len_example();
    // rust_pg::examples::get_nth_word::PgGetNthWord::example();
    // rust_pg::examples::bubble_sort::BubbleSort::example();
    // rust_pg::examples::trait_impl::TraitImpl::run_example();
    // rust_pg::examples::vector::PgVector::run_example();
    // rust_pg::examples::turbo_fish::PgTurboFish::run_example();
    // rust_pg::examples::concurrency::ConcExample::run_example();
    // rust_pg::examples::deref::DerefExample::run_example();
    // rust_pg::examples::my_option::MyOptionExample::run_example();
    // rust_pg::examples::matches::MatchExample::run_example();

    // /* Needs to be extended */
    // rust_pg::examples::core_cell::CoreCellExample::run_example();

    /*
     * TODO remove it when I know how to create libraries
     * without struct instantiation.
     */
    // rust_pg::leet_code::two_sum::Solution::two_sum(
    //     Vec::from([1, 2, 4]),
    //     6,
    // );

    //rust_pg::leet_code::add_two_nums::TestSolution::test_solution();
    /*
     * Future topics
     * 1: std::Option https://doc.rust-lang.org/std/option/
     * 2: How to implement the next method:
     * 3: continue with the project management.
     *
     *
     * PgVector::read_vec::<u64>(vec_ref);
     * PgVector::read_vec::<u64>(vec_slice);
     * fn read_vec<T>(vec_slice : &[T]){
     *   for (idx, &ref_elem) in vec_slice.iter().enumerate(){
     *       println!("vec[{}] = {}", idx, ref_elem);
     *    }
     *  }
     *
     *
     *
     */

    /* option cannot be dereferenced. */
    /* reference to option can be dereferenced. */
}
