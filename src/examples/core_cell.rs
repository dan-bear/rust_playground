/**
 * core::cell
 * https://doc.rust-lang.org/core/cell/index.html
 * Rust borrow-checker allows multiple immutable
 * (read-only) refs to a memory location and a single
 * mutable (writable) refs to a memory as long as there
 * are no immutable refs. Yet, sometimes several mutable
 * references are needed, the core::cell provide several
 * types that allow a safer way for doing so when the
 * memory is accessed from a single thread.
 *
 * Note: the Mutex<T>,RwLock<T>, OnceLock<T> and atomic
 *       types are the sync-mechanisms to allow access
 *       from several threads.
 *
 * core::cell provides the Cell<T>, RefCell<T> and
 * OnceCell<T> discussed in this example.
 *
 *
 *
 * To Read:
 * https://stackoverflow.com/questions/30831037/situations-where-cell-or-refcell-is-the-best-choice     
 *
 *
 *
 * TODO - I DO NOT UNDERSTAND WHEN WOULD I WANT TO USE THE
 * CELL PATTERN. NEED TO FIND AN EXAMPLE.
 */

/**
 * Interior Mutability Pattern:
 *
 * The interior-mutability patter allows to mutate data
 * even when there are immutable references to it.
 *
 * The pattern use unsafe code inside the RefCell
 * structure to do so.
 *
 * It's the programmer responsibility to mutate the
 * data without creating run-time races.
 *
 * The reference rules are checked in run-time when
 * using the RefCell<T> pattern, i.e. the program
 * panics if the rules are broken.
 *
 * RefCell<T> cannot be used for multi-threaded programs!
 *
 * The data (value) inside the RefCell can be mutated
 * also the RefCell itself is immutable. This is called
 * Interior-Mutability pattern.
 *
 *
 *
 *
 * Use case:
 * 1: A library that tracks how close is a value of a
 *    variable to a given limit.
 * 2: The library sends messages to the variable's user
 *    which tells how close the variable's value gets
 *    to the limit.
 * 3: The user has to provide the mechanism for sending
 *    the message. The library demands the user to
 *    implement the Messenger trait.
 */

/**
 * The Messenger trait is used as the interface/contract
 * between the LimitTracker and its user.
 * The LimitTracker gets a reference to a type which
 * implements the Messenger trait, i.e. implements the
 * send() method.
 * The user must implement the Messenger trait so the
 * LimitValueTracker could pass messages to the user.
 */
pub trait Messenger {
    /**
     * The method used by the LimitValueTracker to pass
     * messages to the user.
     */
    fn send(&self, message: &str);
}

/**
 * The limit tracker struct.
 * 1: The struct contains a reference to a generic messenger
 *    passed by the user.
 *   1.1: Since the the messenger is passed by a reference,
 *        its life time needs to be specified, as long as
 *        the LimitTracker is "active", the passed messenger
 *        must be valid, otherwise, the program panics as
 *        the LimitTracker calls its method.
 *   1.2: Note the T: Messenger syntax, the compiler checks
 *        that the passed type implements the Messenger
 *        track.
 */
pub struct LimitTracker<'a, T: Messenger> {
    /** A reference to the messenger */
    m_r_messenger: &'a T,
    /** The value to track */
    m_tracked_val: usize,
    /** The maximal allowed value */
    m_max_val: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(r_messenger: &'a T, max_val: usize) -> LimitTracker<'a, T> {
        return LimitTracker {
            m_r_messenger: r_messenger,
            m_tracked_val: 0,
            m_max_val: max_val,
        };
    }

    /**
     * The set value allows the user to set the value
     * tracked by the LimitTracker.
     *
     * Notes:
     * 1: The send method gets an immutable reference to a
     *    messenger.
     * 2: The set value method does not return a value, so
     *    it's not that straight forward to test it.
     * 3:
     */
    pub fn set_value(&mut self, value: usize) {
        self.m_tracked_val = value;

        let percentage_of_max_val: f64 =
            self.m_tracked_val as f64 / self.m_max_val as f64;

        if percentage_of_max_val >= 1.0 {
            self.m_r_messenger.send("error, values crossed the limit");
        } else if percentage_of_max_val >= 0.9 {
            self.m_r_messenger.send("Warning: crossed the 0.9 mark");
        } else if percentage_of_max_val >= 0.75 {
            self.m_r_messenger.send("Attention: crossed the 0.75 mark");
        }
    }
}

/**
 * Now let's try and test it, by implementing a messenger.
 */

/**
 * The MockMessenger has a vector which can keep the
 * strings passed from the LimitTracker.
 */
struct MockMessengerFirstTry {
    _sent_messages: Vec<String>,
}

impl MockMessengerFirstTry {
    /**
     * The new static method creates a MockMessenger (with
     * an empty vec) and returns its copy.
     */
    fn _new() -> MockMessengerFirstTry {
        MockMessengerFirstTry {
            _sent_messages: vec![],
        }
    }
}

impl Messenger for MockMessengerFirstTry {
    /*
     * What's the issue with the next try?
     * The borrow-checker error is:
     *
     * cannot borrow `self.sent_messages` as mutable,
     * as it is behind a `&` reference `self` is a `&`
     * reference, so the data it refers to cannot be
     * borrowed as mutable
     *
     * The method tries to change sent_messages field, i.e
     * to mutate it. But, the send method expects an
     * immutable reference of the messenger.
     *
     * One way is to change the send() method of the
     * LimitTracker to get a mutable reference to the
     * messenger, but then, the contract between the
     * user (Messenger implementor) and the LimitTracker
     * changes, as the tracker can change the messenger.
     *
     * The send() interface is safer when it does not
     * mutate the messenger.
     *
     */
    // fn send(&self, message: &str) {
    //     self.sent_messages.push(String::from(message));
    // }
    /*
     * Let's implement something to avoid compilation errors.
     */
    fn send(&self, message: &str) {
        let _do_nothing_with_message: &str = message;
    }
}

/*
 * The RefCell<T> is a solution for this problem, instead
 * of having a Vec<String> as a field of the MockMessenger, a
 * RefCell<Vec<String>> can be used. Let's work it out:
 */
pub struct MockMessenger {
    m_sent_messages_ref_cell: core::cell::RefCell<Vec<String>>,
}

impl MockMessenger {
    /*
     * Note that the refcell is created with an empty
     * vector.
     */
    fn new() -> MockMessenger {
        return MockMessenger {
            m_sent_messages_ref_cell: core::cell::RefCell::new(vec![]),
        };
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.m_sent_messages_ref_cell
            .borrow_mut()
            .push(String::from(message));
    }
}

pub struct CoreCellExample;

impl CoreCellExample {
    pub fn run_example() {
        CoreCellExample::cell();
        CoreCellExample::ref_cell();
        crate::core::utils::Utils::print_line_separator();
    }

    fn cell() {
        /*
         * The cell wraps/obtain/contain a value.
         * The value rules are:
         * 1: Mutable reference (&mut T) to the inner value
         *    of the cell can never be obtained.
         *
         * 2: The cell's value itself cannot be obtained
         *    without replacing it with another value.
         *
         * The rules ensure there is never more than one
         * reference pointing to the inner value.
         *
         * Cell<T> methods:
         * 1: If T implements te Copy trait, Cell<T>
         *    provides a get() method which duplicates
         *    the inner value and returns it (copy).
         *
         * 2: If T implements the Default (value) trait,
         *    Cell<T> provides a take() method which
         *    replaces the current inner value to the
         *    default value and returns the replaced value.
         *
         * 3: replace(): replaces the current inner value
         *    with a given value and returns the current
         *    inner value.
         *
         * 4: into_inner(): consumes the Cell<T> and return
         *    the inner value.
         *
         * 5: set(): replaces the inner value and drops it.
         *
         * Note: use cell with "light" types, since it
         * involved copying the data.
         */
        let _imt_local_var: u64 = 2;
        let mut _mt_local_var: u64 = 3;
        let u64_cell = core::cell::Cell::<u64>::new(5);
        //imt_local_var = 7 /* Error! immutable variable */
        _mt_local_var = 11; /* Fine, mutable variable*/
        /* Also cell is immutable, the inner value can change */
        u64_cell.set(13);
    }

    fn ref_cell() {
        let mock_messenger: MockMessenger = MockMessenger::new();
        let mut limit_tracker: LimitTracker<'_, MockMessenger> =
            LimitTracker::new(&mock_messenger, 10 as usize);

        /*
         * Let's borrow the Vec<String> wrapped in the
         * RefCell to see the messages.
         *
         * Note that each borrow is a immutable borrow, as
         * the code oly reads the messages sent by the
         * tracker. (The LimitTracker is the one writing
         * to to the Vec<String> using the send() method).
         */
        limit_tracker.set_value(1);
        assert_eq!(
            mock_messenger.m_sent_messages_ref_cell.borrow().len(),
            0
        );
        limit_tracker.set_value(8);
        assert_eq!(
            mock_messenger.m_sent_messages_ref_cell.borrow().len(),
            1
        );
        println!(
            "{}",
            mock_messenger.m_sent_messages_ref_cell.borrow()[0]
        );
        limit_tracker.set_value(9);
        assert_eq!(
            mock_messenger.m_sent_messages_ref_cell.borrow().len(),
            2
        );
        println!(
            "{}",
            mock_messenger.m_sent_messages_ref_cell.borrow()[1]
        );
        limit_tracker.set_value(10);
        assert_eq!(
            mock_messenger.m_sent_messages_ref_cell.borrow().len(),
            3
        );
        println!(
            "{}",
            mock_messenger.m_sent_messages_ref_cell.borrow()[2]
        );

        /*
         * Now let's make the system panic by having two
         * mutable borrows through the RefCell.
         * Note - two mutable references in the same scope
         * do not lead to borrow-checker error.
         */
        let mut _r_messages1 =
            mock_messenger.m_sent_messages_ref_cell.borrow_mut();
        /*
         * Uncomment and run to make the program panic.
         */
        // let mut _r_messages2 =
        //     mock_messenger.m_sent_messages_ref_cell.borrow_mut();
        /*
         * The program panics in the previous line, as
         * the RefCell mechanism two calls to borrow_mut()
         * in the same scope.
         * The next lines just tries to emphasis what one
         * might like to do.
         */
        // _r_messages1.push(String::from("not cool 1"));
        // _r_messages2.push(String::from("not cool 2"));

        /*
         * NOTE: RefCell can be thought of a single-thread
         * rw-lock for some data as it causes the program
         * to panic when:
         * 1. The variable is mutated by more than a single
         *    writer.
         * 2. The variable is mutated while it's also read.
         */
        let protected_data: core::cell::RefCell<u64> =
            core::cell::RefCell::<u64>::new(2);
        let mut _r_mt_u64_val: core::cell::RefMut<'_, u64> =
            protected_data.borrow_mut();
        /*
         * Next line causes a panic since the RefCell
         * detects a immutable borrow while the reference
         * is already mutably borrowed.
         */
        // let r_imt_u64_val: core::cell::Ref<'_, u64> =
        //     protected_data.borrow();
        // println!("protected value is: {}", *r_imt_u64_val);
        // *_r_mt_u64_val += 1;
        // println!("protected value is: {}", protected_data.borrow());

        /*
         * TODO - need to understand what's going on inside
         * the ref-cell.
         */
    }
}
