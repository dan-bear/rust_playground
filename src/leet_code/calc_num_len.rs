pub fn calc_num_len(mut num: u64) -> u8 {
    let mut len: u8 = 0;
    if num == 0  {
        len = 1;
    } else {
        while num != 0 {
            num /= 10;
            len += 1;
        }
    }
    return len;
}

pub fn run_example(){
    let num: u64 = 31415926535;
    let len: u8 = calc_num_len(num);
    println!("num is: {num} its len is: {len}");   
}