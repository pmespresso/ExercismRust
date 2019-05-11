// s = 1 -> 1
// s = 2 -> 2
// s = 3 -> 4
// s = 4 -> 8
// s = 5 -> 16
// s = 6 -> 32

pub fn square(s: u32) -> u64 {
    match s == 0 || s > 64 {
        true => panic!("Square must be between 1 and 64"),
        false => 2u64.pow(s - 1)
    }
}

pub fn total() -> u64 {
    (1..=64).map(|i| square(i)).sum()
}
