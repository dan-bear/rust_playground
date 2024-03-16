pub fn pg_pointers() {
    println!("POINTERS");
    pg_box();
    pg_raw_pointers();
    crate::core::utils::Utils::print_line_separator();
}

//_____________________________________________________________________________
fn pg_box() {
    pg_box_basic();
    pg_box_deref();
    pg_box_drop();
}

fn pg_box_basic() {
    println!("box basics");
    let u64_box: Box<u64> = Box::new(2);
    //note that the u64_box is not dereferenced like in c.
    println!("value in u64 box is: {u64_box}");
    //When u64_box goes out of scope, drop() is called and
    //the memory is deallocated from the heap.
}

fn pg_box_deref() {
    println!("box deref");
    pg_box_deref_part1();
    pg_box_deref_part2();
}

fn pg_box_deref_part1() {
    println!("box deref part1:");
    let x: u64 = 2;
    let box_y: Box<u64> = Box::new(2);
    //assert_eq!(x, box_y);//doesn't compile since u64 and
    //Box<u64> are not the same type.
    assert_eq!(x, *box_y); //derefer the u64 in the Box<u64>
}

//______________________________________________________________________________
//PG_BOX_DEREF_PART2() function.

//The tuple struct way as in the rust book.
//see below the regular struct way.
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        return MyBox(x);
    }
}

// The regular struct way.
// struct MyBox<T> {
//     m_type: T,
// }
// impl<T> MyBox<T> {
//     fn new(t: T) -> MyBox<T> {
//         return MyBox { m_type: t };
//     }
// }

use std::ops::Deref; //The standard library deref trait.
impl<T> Deref for MyBox<T> {
    //associated type, will be explained later.
    type Target = T;
    fn deref(&self) -> &Self::Target {
        return &self.0; //the first member of the tuple
                        //struct.
    }
}

fn pg_box_deref_part2() {
    println!("box_deref_part2");
    let my_box: MyBox<u64> = MyBox::new(2);
    let x: u64 = 2;
    assert_eq!(x, *my_box);
    println!(
        "x value is: {x}, my_box dereferenced value is: {}",
        *my_box
    );
}
//______________________________________________________________________________
//PG_BOX_DROP
struct CustomString {
    data: String,
}

//The drop trait (unlike the deref trait) is included in
//the prelude.
impl Drop for CustomString {
    /**
     * Note: drop() gets a mutable reference to self. As
     * part of the ownership rules, it means there are no
     * other mutable or immutable reference to self
     * available. Which promises drop is called once and
     * only once.
     */
    fn drop(&mut self) {
        println!(
            "The dropped data from custom string is: {}",
            self.data
        );
    }
}

fn pg_box_drop() {
    pg_box_simple_drop();
    pg_box_early_drop();
}

fn pg_box_simple_drop() {
    let custom_string: CustomString = CustomString {
        data: String::from("3.1415"),
    };
    println!("custom string data is: {}", custom_string.data);
    //Now it's gonna drop.
}

fn pg_box_early_drop() {
    // sometimes drop needs to be called before the
    // variable goes out of scope.
    let custom_str = CustomString {
        data: String::from("2.7182818284"),
    };
    // Cannot drop the custom string since the automatic
    // drop call cannot be avoided and "double drop (free)"
    // is considered a bug.
    // to allow it, the std::mem::drop function. The
    // function is in the prelude, so it can be used as
    // follows:
    drop(custom_str);
    println!("custom str was dropped before the scope ended");
}

fn pg_raw_pointers() {
    //Two types of raw pointers.
    //immutable: *const T.
    //mutable: *mut T.
    let mut num: u64 = 2;
    //make two pointers pointing to stack by using a
    //reference to a value saved on stack.
    let imt_ptr: *const u64 = &num as *const u64;
    let mut_ptr: *mut u64 = &mut num as *mut u64;

    //It's ok to create raw pointers in safe code, but the
    //rpointers cannot be dereferenced otherwhere than
    //in an unsafe block.
    unsafe {
        /*
         * print the value pointed by the pointers and the
         * memory address (stack address) which is the
         * pointers' values.
         */
        println!(
            "imt_ptr value is {:p} and it points to {}",
            imt_ptr, *imt_ptr
        );
        println!(
            "mut_ptr value is {:p} and it points to {}",
            mut_ptr, *mut_ptr
        );
    }

    /*
     * A raw pointer can be created from an integral value.
     * i.e. make a pointer for an arbitrary address.
     */
    let mem_addr: u64 = 0x12345;
    let imt_mem_ptr: *const u64 = mem_addr as *const u64;
    println!(
        "imt_mem_ptr value is: {:p}, might not be an accessible address",
        imt_mem_ptr
    );
}
