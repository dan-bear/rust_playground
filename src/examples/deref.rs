pub struct DerefExample;

impl DerefExample {
    /*
     * Implementing the Deref trait allows us to define
     * the behavior of the dereference operator, *
     */
    pub fn run_example() {
        DerefExample::value_from_reference();
        DerefExample::value_from_pointer();
        DerefExample::deref_my_box();
        DerefExample::deref_coercion();
        crate::core::utils::Utils::print_line_separator();
    }

    fn value_from_reference() {
        let x: u64 = 2;
        let y: &u64 = &x;
        let z = *y;
        assert!(x == 2);
        assert!(z == x);
        assert!(*y == x);
        println!("x = {}, z = {}", x, z);
        /*
         * Note: the value of x is not moves to z but
         * copied. Thus, it's possible to use both x and z
         * in the println!().
         */
    }

    fn value_from_pointer() {
        let x: u64 = 2;
        let p_x: Box<u64> = Box::new(x);
        assert!(x == *p_x);
        println!("x = {}, *p_x = {}", x, p_x);
    }
}

struct MyBox<T>(T); /* tuple struct */

impl<T> MyBox<T> {
    fn new(t_val: T) -> MyBox<T> {
        return MyBox(t_val);
    }
}

/**
 * Deref trait definition:
 * pub trait Deref{
 *     type Target: Sized?;
 *     
 *     fn deref(&self) -> &Self::Target;
 * }
 */
impl<T> std::ops::Deref for MyBox<T> {
    /**
     * Tell Deref that the associated type is T
     * (fulfill the type Target: Sized? syntax).
     */
    type Target = T;

    /**
     * Given a reference to a MyBox object, a reference to
     * the internal value is returned.
     *
     * Reminder, in tuple structs, the members are related
     * using their order (0 for the first member, 1 for the
     * second...).
     *
     * Note: deref returns a reference due to the ownership
     * rules. If the return type is T, then the value is
     * moved from the object, i.e. self cannot access the
     * value anymore.
     *
     * Note: an immutable reference is returned, thus, the
     * value cannot be changed through the reference.
     */
    fn deref(&self) -> &T {
        return &self.0;
    }
}

impl DerefExample {
    fn deref_my_box() {
        let p_val: MyBox<u64> = MyBox::new(2);
        let val = *p_val;
        assert!(val == 2);
        /*
         * Can still use the p_val, as deref returns a
         * reference to the value inside the box.
         */
        let same_val = *p_val;
        assert!(val == same_val);
    }

    fn deref_coercion() {
        /*
         * coercion: the practice of persuading someone to
         * do something by using force or threats (forcing).
         */

        fn print(a_string: &str) {
            println!("{}", a_string);
        }

        let my_str_box: MyBox<String> = MyBox::new(String::from("Hello"));
        /*
         * Deref Coercion allows to call the print function
         * without passing a string or a string slice, but
         * with a reference to a type which implements 
         * deref and deref() return value is &str or &String.
         */
        print(&my_str_box);
        /*
         * Without deref coercion.
         */
        print(&(*my_str_box));
        /*
         * And if rust compiler had not allowed to pass 
         * String to a function expecting a string slice
         * the next syntax would be needed.
         * Reminder [..] creates a slice from the whole
         * string. 
         */
        print(&(*my_str_box)[..]);

        /*
         * The deref coercion can occur:
         * 1: from &T to &U when T implements Deref<Target=U>
         * 2: from &mut T to &mut U when T implements DerefMut<Target=U>
         * 3: from &mut T to &U when T implements Deref<Target=U> 
         */
    }
}

/*
 * https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch15-02-deref.html
 */
