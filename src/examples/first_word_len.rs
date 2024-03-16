pub fn pg_first_word_len_example() {
    println!("FIRST WORD LEN EXAMPLE");
    let str1 = String::from("3.14159265358979323");
    let str2 = String::from("pi is not that tasty");
    println!(
        "\"{str1}\" number of chars till space is: {}",
        first_word_len(&str1)
    );
    println!(
        "\"{str2}\" number of chars till space is: {}",
        first_word_len(&str2)
    );
    crate::core::utils::Utils::print_line_separator();
}
/**
 * @param r_string a reference to a string or a string
 * slice.
 * @return the length of the first word in a string.
 * @note usize is architecture depend, it's an unsigned
 * integer of the same width of a pointer (in a 64 bit
 * architecture for example, usize is equivalent to u64).
 */
fn first_word_len(r_string: &str) -> usize {
    let len: usize = first_word_len_ver1(r_string);
    assert_eq!(len, first_word_len_ver2(r_string));
    assert_eq!(len, first_word_len_ver3(r_string));
    return len;
}

/**
 * @param r_string a reference to a string or a string slice.
 * @return the length of the first word in a string.
 */
fn first_word_len_ver1(r_string: &str) -> usize {
    let mut char_idx: usize = 0;
    let byte_arr: &[u8] = r_string.as_bytes();
    for r_char in byte_arr {
        if *r_char == b' ' {
            return char_idx;
        }
        char_idx += 1;
    }
    assert_eq!(char_idx, r_string.len());
    return char_idx;
}

/**
 * @param r_string a reference to a string or a string slice.
 * @return the length of the first word in a string.
 */
fn first_word_len_ver2(r_string: &str) -> usize {
    let byte_arr: &[u8] = r_string.as_bytes();
    for char_idx in 0..r_string.len() {
        if byte_arr[char_idx] == b' ' {
            return char_idx;
        }
    }
    return r_string.len();
}

/**
 * @param r_string a reference to a string or a string slice.
 * @return the length of the first word in a string.
 */
fn first_word_len_ver3(r_string: &str) -> usize {
    let byte_arr: &[u8] = r_string.as_bytes();
    for (char_idx, &r_char) in byte_arr.iter().enumerate() {
        if r_char == b' ' {
            return char_idx;
        }
    }
    return r_string.len();
}
