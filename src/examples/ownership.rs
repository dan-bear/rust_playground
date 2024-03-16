pub fn pg_ownership() {
    println!("OWNERSHIP");
    pg_assignment_ownership();
    pg_function_ownership();
    crate::core::utils::Utils::print_line_separator();
}

//_____________________________________________________________________________
fn pg_function_ownership() {
    println!("pg_function_ownership()");

    //______________________________________________________
    //Basic ownership passing - immutable to immutable.
    let imt_str0: String = String::from("31415926535");
    //The ownership is passed, imt_str cannot be used anymore!
    pg_function_ownership_aux1(imt_str0);
    //Cannot use imt_str0 any longer (String::drop() was
    //called when the function execution was completed).

    //______________________________________________________
    //Basic ownership passing - immutable to mutable.
    //Since the ownership of imt_str1 is passed to the
    //function, the variable does not need to be mutable.
    //I general, ownership allows to control mutability, so
    //the compiler rises a warning in case imt_str1 is
    //defined here as mutable.
    let imt_str1: String = String::from("31415926535");
    pg_function_ownership_aux2(imt_str1);
    //Cannot use imt_str1 any longer (String::drop() was
    //called when the function execution was completed).

    //______________________________________________________
    //Getting back the ownership.
    let mut imt_str2: String = String::from("31415926535");
    imt_str2 = pg_function_ownership_aux3(imt_str2);
    println!("The string can be reused. str is: {imt_str2}");

    //______________________________________________________
    //Borrow an immutable-variable ownership using a reference.
    let str_len: u64 = pg_function_ownership_get_str_len(&imt_str2);
    //can still use the string as it was borrowed by the method.
    println!("The string is: {imt_str2} and its length is {str_len}");

    //______________________________________________________
    //Borrow a mutable reference.
    //1: Cannot pass a mutable reference to an immutable
    //   variable (compilation error).
    //2: References are immutable by default, so a mutable
    //   reference must be passed.
    let mut mt_str3: String = String::from("3.1415926535");
    let len_after_push: u64 =
        pg_function_ownership_push_and_get_len(&mut mt_str3);
    println!("str after push is: {mt_str3}, its length is: {len_after_push}");
    crate::core::utils::Utils::print_line_separator();
}

//The string ownership is moved to the method.
//Note that the passed string cannot be mutate since the
//parameter is not explicitly mutable.
fn pg_function_ownership_aux1(a_string: String) {
    println!("owned string is: {a_string}");
}

fn pg_function_ownership_aux2(mut a_string: String) {
    a_string.push_str("8979323");
    println!("The string is: {a_string}");
}

fn pg_function_ownership_aux3(a_string: String) -> String {
    println!("input string is: {a_string}");
    println!("return the string ownership to the caller");
    return a_string;
}

fn pg_function_ownership_get_str_len(a_ref_string: &String) -> u64 {
    return a_ref_string.len() as u64;
}

fn pg_function_ownership_push_and_get_len(
    a_ref_string: &mut String,
) -> u64 {
    a_ref_string.push_str("8979323");
    return a_ref_string.len() as u64;
}

//_____________________________________________________________________________
fn pg_assignment_ownership() {
    println!("pg_assignment_ownership()");

    //Each value in rust has an owner
    //(a heap-memory-location is a better term?).
    //Each value (memory location) in rust can have a single
    //owner at any given time.
    //When the owner (variable) goes out of scope, the value
    //(memory cell) is dropped.

    {
        //When using assignment with string, the string's
        //data is not copied. The String struct has
        //(at least) two members: m_ptr_to_heap, m_str_len
        //Those, are the values which are copied when
        //using assignment. To cut a long story short, it's
        //a shallow copy and not a deep copy. The ownership
        //mechanism promises that no two variables owns the
        //same heap-mem-address. So, when using assignment,
        //ownership is passed!.
        let str1: String = String::from("3.1415926535");
        let mut str2: String = str1;
        //from now on, str1 cannot be used and it's enforced
        //by the compiler.
        println!("str2 is: {str2}"); //doing the same with
                                     //str1 would lead to
                                     //compilation error!.
        str2.push_str("8979323");
    }

    {
        //To allow deep copy of a string, use clone.
        let immut_str1: String = String::from("3.14159265358979323");
        let immut_str2: String = immut_str1.clone();
        println!("str1: {immut_str1}");
        println!("str2: {immut_str2}");
    }
    crate::core::utils::Utils::print_line_separator();
}
