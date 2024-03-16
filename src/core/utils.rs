pub struct Utils;

impl Utils {
    pub fn print_line_separator() {
        let line_len: u8 = 80;
        println!(
            "{}",
            Utils::print_len_specific_line_separator(line_len)
        );
    }

    fn print_len_specific_line_separator(len: u8) -> String {
        let line_str: String = String::from("_").repeat(len as usize);
        return line_str;
    }
}
