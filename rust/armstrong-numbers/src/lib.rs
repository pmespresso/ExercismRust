pub fn is_armstrong_number(num: u32) -> bool {
    let digits_string = num.to_string();
    let power: u32 = digits_string.len() as u32;

    let sum: u32 = digits_string
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| d.pow(power))
        .sum();

    sum == num
}
