/**
 * Complex num struct.
 */
pub struct Complex {
    m_real: i64,
    m_img: i64,
}
/**
 * Complex num implementation.
 */
impl Complex {
    pub fn get_real(&self) -> i64 {
        return self.m_real;
    }

    pub fn get_img(&self) -> i64 {
        return self.m_img;
    }

    pub fn _set_real(&mut self, real_val: i64) {
        self.m_real = real_val;
    }

    pub fn _set_img(&mut self, img_val: i64) {
        self.m_img = img_val;
    }
}

/**
 * Implementing PartialEq trait for complex allows us
 * to support the eq() [==] and ne() [!= ] operators.
 *
 * In this example, the only thing the implementation
 * is about to do is to compare field by field between
 * two complex instances.
 *
 * Note: Since it's a common practice (fieldwise comparison)
 * rust allows us to use the next syntax:
 *
 * #[derive(PartialEq)]
 * struct YouStructName{... struct's fields ...}
 *
 * This syntax make the compiler to generate the PartialEq
 * implementation code which checks whether the fields are
 * equal to each other.
 *
 */
impl PartialEq for Complex {
    fn eq(&self, other: &Complex) -> bool {
        return self.get_real() == other.get_real()
            && self.get_img() == other.get_img();
    }
    /*
     * PartialEq trait include two functions, eq (==) and ne
     * (!=), if the only thing one wants is to make ne()
     * being the negation of eq() [which is very common],
     * the rust compiler generates the code for ne
     * automatically so the next implementation can be
     * avoided (and it's commented out).
     */
    // fn ne(&self, other: &Complex) -> bool{
    //     return !self.eq(other);
    // }
}

/**
 * Implement the fmt::Debug so one can print the complex
 * number in debug-mode (using the {:?}).
 *
 * Note: rust also gives us the ability to use:
 *
 * #[derive(Debug)]
 * struct YourStructName {... your struct fields...}
 *
 * When using it, the compiler implements the standard
 * std::fmt::Debug method for complex. The standard output
 * is a line containing the field names and their values,
 * for example:
 *
 * #[derive(Debug)]
 * struct MyStruct{
 *     m_member : u64
 * }
 *
 * fn func(){
 *    let my_struct MyStruct : MyStruct{m_member: 2};
 *    my_struct.m_member = 3;
 *    println!("{:?}", my_struct)
 * }
 *
 * The println! output will be: "MyStruct { m_member : 3 }".
 *
 * Note: another syntax is println!("{my_struct:?}").
 *
 * In this example, the standard implementation does not
 * provides our needs.
 */
impl std::fmt::Debug for Complex {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        return write!(
            formatter,
            "{} + i{}",
            self.get_real(),
            self.get_img()
        );
    }
}

/**
 * The Clone trait is use to duplicate (create a copy) of
 * the object.
 *
 * When using the derive syntax:
 * #[derive(Clone)]
 *
 * The compiler generates the standard clone() method which
 * creates a new struct which is copied field by field (
 * shallow copy).
 *
 * So the next code could have been removed by adding the
 * #[derive(Clone)] above the Complex struct definition.
 */

impl Clone for Complex {
    /**
     * clone() must be implemented when implementing the
     * Clone trait.
     */
    fn clone(&self) -> Complex {
        let complex_copy = Complex {
            m_real: self.get_real(),
            m_img: self.get_img(),
        };
        /*
         * Note, the method returns a copy of the data
         * written on stack during this method execution.
         */
        return complex_copy;
    }

    /*
     * Another method provided automatically when clone()
     * is implemented is the clone_from() method.
     * It's implemented here,
     */
    fn clone_from(&mut self, r_source: &Complex) {
        *self = r_source.clone();
    }
}

pub struct TraitImpl;

impl TraitImpl {
    pub fn run_example() {
        TraitImpl::partial_eq_and_display_example();
        TraitImpl::clone_example();
        crate::core::utils::Utils::print_line_separator();
    }

    fn partial_eq_and_display_example() {
        let comp1 = Complex {
            m_real: 2,
            m_img: 3,
        };
        let comp2 = Complex {
            m_real: 3,
            m_img: 5,
        };
        let comp3 = Complex {
            m_real: 2,
            m_img: 3,
        };

        /*
         * Note there is no difference in the println!
         * syntaxes.
         */
        if comp1 != comp2 {
            println!("{:?} != {:?}", comp1, comp2);
        }
        if comp1.ne(&comp2) {
            println!("{comp1:?} ne {comp2:?}");
        }

        if comp1 == comp3 {
            println!("{comp1:?} == {comp3:?}");
        }

        if comp1.eq(&comp3) {
            println!("{:?} eq {:?}", comp1, comp3)
        }
    }

    fn clone_example() {
        let comp0: Complex = Complex {
            m_real: 2,
            m_img: 3,
        };
        let comp1: Complex = comp0.clone(); /* <= clone explicitly. */
        let mut comp2: Complex = Complex {
            m_real: 0,
            m_img: 0,
        };
        comp2.clone_from(&comp0);
        println!("Cloned comps are the same! {comp0:?}, {comp1:?}, {comp2:?}");

        /*
         * Note: clone() is needed in rust (unlike
         * copy-assignment in other languages) since
         * assignment moves the ownership   !
         */

        let _comp3: Complex = comp0; /* <= comp0's data moved to comp3 */
        /* 
        * Using comp0 now raises a borrow-checker error.
        * Thus, 
        */
        
    }
}
