pub fn is_prime(n: u32) -> bool {
    !(2..=(n as f64).sqrt() as u32).any(|m| n % m == 0)
}

pub fn nth(n: u32) -> u32 {
    match n {
        n if n < 0 => 2,
        n => (1..).filter(|num| is_prime(*num)).nth((n + 1u32) as usize).unwrap(), 
    }
    // note: nth here is a method on type Vec from the filter, not a recursion
}
