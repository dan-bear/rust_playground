pub struct FirstSteps;

impl FirstSteps {
    pub fn run_example() {
        FirstSteps::pg_datatype();
        FirstSteps::pg_mutable();
        let _ = FirstSteps::pg_func(7); //To ignore a return value.
        FirstSteps::pg_ifelse(true, 2, 3);
        FirstSteps::pg_ifelse(false, 2, 3);
        FirstSteps::pg_loop();
        FirstSteps::pg_while();
        FirstSteps::pg_for();
        FirstSteps::pg_basic_str();
        FirstSteps::structs();
    }

    fn pg_basic_str() {
        println!("pg_basic_str()");
        {
            //Heap allocated, will be dropped when the param is out
            //of scope.
            let immut_str_val: String = String::from("3.1415926535");
            println!("Almost pi: {immut_str_val}");
        } //RAII style.
          //String::drop() method is called (implicitly). It's
          //the String module/library/package has to implement
          //the drop() method so it releases the heap memory.

        {
            //Mutable string which can be modified.
            let mut mut_str_var: String = String::from("3.1415926535");
            mut_str_var.push_str("8979323");
            println!("Almost pi: {mut_str_var}");
        } //RAII style.
        crate::core::utils::Utils::print_line_separator();
    }

    fn pg_for() {
        println!("pg_for()");
        let array: [u64; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
        let mut idx: u64 = 0;
        for elem in array {
            println!("elem in index {idx} is {elem}");
            idx += 1;
        }

        for idx in 0..7 {
            println!("elem in index {idx} is {}", array[idx]);
        }
        crate::core::utils::Utils::print_line_separator();
    }

    fn pg_while() {
        println!("pg_while()");
        let mut var: u64 = 2;
        while var < 11 {
            println!("var value is: {var}");
            var += 1;
        }
        println!("var value is: {var}");
        crate::core::utils::Utils::print_line_separator();
    }

    fn pg_loop() {
        println!("pg_loop()");
        let mut var: u64 = 2;
        loop {
            println!("in loop: var value is: {var}");
            var += 1;
            if var == 5 {
                continue;
            }
            var += 1;
            if var == 7 {
                break;
            }
        }
        println!("var last value is: {var}");
        crate::core::utils::Utils::print_line_separator();
    }

    fn pg_ifelse(condition: bool, val_if_true: u64, val_if_false: u64) {
        println!("pg_ifelse()");
        //note, any block's value is the last expression in the block.
        let res: u64 = if condition { val_if_true } else { val_if_false };
        println!("res is: {}", res);
        crate::core::utils::Utils::print_line_separator();
    }

    //function playground.
    fn pg_func(param: u64) -> u64 {
        println!("pg_func()");
        println!("param is: {}", param);
        let mut res: u64 = 5;
        res += param;
        println!("retrun value is {}", res);
        crate::core::utils::Utils::print_line_separator();
        return res; //I know one could use res. But for now, I don't like it.
                    //What about casting?
    }

    //mutable playground
    fn pg_mutable() {
        println!("pg_mutable()");
        let var0: u64 = 2; //imutable by default.
        let mut var1: u64 = 3; //mutable.
                               //var0 += 1; //compilation error, since var0 is immutable.
        var1 += 1; //life is good.
        println!("var0 = {}, var1 = {}", var0, var1);
        crate::core::utils::Utils::print_line_separator();
    }

    //data type playground:
    fn pg_datatype() {
        println!("pg_datatype()");
        let var0: i32 = -1;
        let var1: u32 = 1;
        println!("var0 = {}, var1 = {} ", var0, var1);
        crate::core::utils::Utils::print_line_separator();
    }

    fn structs() {
        #[derive(Debug)]
        struct MyStruct {
            m_x: u64,
            m_y: u32,
        }
        let my_struct = MyStruct { m_x: 2, m_y: 3 };
        let x: u64 = my_struct.m_x + my_struct.m_y as u64;
        println!("MY STRUCT IS {:?} and its some is: {x}", &my_struct);
    }
}
