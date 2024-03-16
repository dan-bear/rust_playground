pub struct PgVector;

impl PgVector {
    pub fn run_example() {
        /*
         * https://doc.rust-lang.org/std/vec/struct.Vec.html
         */

        PgVector::structure();
        PgVector::new_vec_and_push();
        PgVector::vec_from();
        PgVector::vec_basic_methods();
        PgVector::vec_macro();
        PgVector::vec_slices();

        crate::core::utils::Utils::print_line_separator();
    }

    fn structure() {
        /*
         * A vector is a continuous growing array
         * The vector structure in std is:
         * pub struct Vec<T, A = global>
         * where A is the allocator and it's defaulted to
         * the global allocator so when using rust's default
         * heap allocator there is no need to declare the
         * allocator when a vector is initialized (see
         * initialization examples in basics() method).
         * Potentially, one could create its own allocator
         * and use rust's vector.
         */

        /*
         * Vector as a struct contains three members:
         * 1: pointer to the memory where the data is
         *    stored.
         * 2: capacity - vector's potential number of
         *    elements.
         * 3: length - number of elements in the vector.
         *
         * 
         * https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#396
         * struct Vec<T, A>{
         *     buf: RawVec<T,A>
         *     len: usize
         * }
         * 
         * https://doc.rust-lang.org/beta/src/alloc/raw_vec.rs.html
         * struct RawVec<T,A>{
         *     ptr: Unique<T>,
         *     cap: usize,
         *     alloc: A,
         * }
         * 
         * 
         * https://docs.rs/unique/latest/src/unique/lib.rs.html#5-176
         * struct Unique<T: ?Sized>(NonNull<T>, PhantomData<T>)
         * 
         */

        /*
         * When the length exceeds the capacity, new memory
         * is allocated to increase the capacity is
         * allocated and all the data is copied to the new
         * allocated memory. (Resize => alloc, copy, free).
         */
    }

    fn new_vec_and_push() {
        /* Allocating a new u64 vector  */
        //TODO should one use Vec::<type>::new()? how does
        //the compiler infer what's the type? Maybe it's non
        //of importance when using new.
        let mut vec: Vec<u64> = Vec::new();
        /*
         * Note: the vector does ot allocate memory till
         * data is pushed to it.
         */

        /* Push inserts the value to the end of the vec */
        vec.push(2);
        vec.push(3);
        vec.push(5);
        for (idx, item) in vec.iter().enumerate() {
            println!("vector[{}] = {}", idx, item);
        }
        /* Note: drop is called when the scope ends */

        /*
         * Vector can be initialized with the turbofish
         * feature (see src/examples/turbo_fish.rs).
         */
        let _turbo_fish_vec = Vec::<u32>::new();
    }

    fn vec_from() {
        let _vec_from: Vec<u64> = Vec::from([2, 3, 5, 7]);
        /* Turbofish approach */
        let _turbofish_vec_from = Vec::<u64>::from([2, 3, 5, 7]);

        //Also in turbofish
    }

    fn vec_basic_methods() {
        let mut vec: Vec<u64> = Vec::from([2, 3, 5, 7]);

        /* Access and assign element using [] */
        println!("vec[2] = {}", vec[2]);
        assert_eq!(vec[2], 5);
        vec[0] = 0;
        assert_eq!(vec[0], 0);
        vec[0] = 2;
        assert_eq!(vec[0], 2);
        /*
         * Note: accessing outside the vec boundaries panics
         * the program.
         * Don't do that at home:
         */
        //vec[17] = 0; <= result in program panic.

        /* Regular iterator */
        for r_elem in &vec {
            /*
             * Note:
             * using "for elem in vec{}" also works, but the
             * elements in the vector are copied during the
             * iteration and not referenced for read access.
             * For u64 it does not matter since references
             * demand at least 8 bytes. But for bigger
             * structs, it's important to use reference.
             */
            println!("{r_elem}");
        }
        /* vec.len() -> usize */
        let vec_len: usize = vec.len();
        println!("Vec length is: {vec_len}");

        /* vec.pop() */
        let last_vec_elem: Option<u64> = vec.pop();
        match last_vec_elem {
            Some(elem) => println!("Popped {elem} from vec"),
            None => println!("Tried to pop an empty vector"),
        }

        /* vec.extend */
        vec.extend([7, 11, 13, 17]);
        for (idx, &elem) in vec.iter().enumerate() {
            println!("vec[{}] = {}", idx, elem);
        }
    }

    fn vec_macro() {
        /*
         * It can also initialize each element of a Vec<T>
         * with a given value. This may be more efficient
         * than performing allocation and initialization in
         * separate steps, especially when initializing a
         * vector of zeros:
         */
        /*
         * Create a vector initialized with 3 elems which
         * are all 0s.
         */
        let macro_vec: Vec<u64> = vec![0; 3];
        assert_eq!(macro_vec, [0, 0, 0]);
        /*
         * An equivalent but potentially slower syntax is:
         */
        let mut vec: Vec<u64> = Vec::with_capacity(3);
        vec.resize(3, 0);
        assert_eq!(vec, macro_vec);
    }

    fn vec_slices() {
        /*
         * A slice to a vector gives a read-only access
         * to the vector.
         * It is idiomatic in rust to pass a vector-slice as
         * a parameter to a function which just reads the
         * vector, rather than passing the vector itself.
         */
        let vec: Vec<u64> = Vec::from([2, 3, 5]);
        let vec_ref: &Vec<u64> = &vec;
        let vec_slice: &[u64] = &vec[0..3];
        PgVector::read_u64_vec(vec_ref);
        PgVector::read_u64_vec(vec_slice);
    }

    fn read_u64_vec(vec_slice: &[u64]) {
        for (idx, &ref_elem) in vec_slice.iter().enumerate() {
            println!("vec[{}] = {}", idx, ref_elem);
        }
    }
}
