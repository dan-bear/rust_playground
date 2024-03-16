pub fn get_max_digit(mut num : i64) -> i64{
    let mut max_digit : i64 = 0;
    while num != 0 {
        max_digit = std::cmp::max(max_digit, num % 10);
        num /= 10;
    }
    return max_digit;
}

pub fn run_example(){
    let num1 : i64 = 123456789;
    let num2 : i64 = 0;
    println!("num = {num1}, max digit = {}", get_max_digit(num1));
    println!("num = {num2}, max digit = {}", get_max_digit(num2));
}