pub fn pg_unsafe_functions() {
    println!("UNSAFE FUNCTIONS");
    pg_unsafe_func_basic_syntax();
    crate::core::utils::Utils::print_line_separator();
}

//______________________________________________________________________________
/* Use the unsafe keyword to declare a unsage function. */
unsafe fn dangerous() {
    println!("Unsafe function!!!!");
}

fn pg_unsafe_func_basic_syntax() {
    /* Call unsage func must happen from an unsafe block */
    unsafe {
        dangerous();
    }
}
