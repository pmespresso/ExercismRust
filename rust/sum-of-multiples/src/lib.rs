
pub fn sum_of_multiples(limit: u32, factors:&[u32]) -> u32 {
    (1..limit).filter(|i| factors.iter().any(|f| f != &0u32 && i % f == 0)).sum()
}


// Naive attempt
// fn multiples(factor: u32, limit: u32) -> u32 {
//     let mut result: Vec<u32> = Vec::new();
//     let mut multiplier: u32 = 1;
//     let mut next_multiple: u32 = factor;
//     while next_multiple <= limit {
//         result.push(next_multiple);
//         next_multiple = multiplier * factor;
//         multiplier += 1;
//     }

//     result.iter().sum()
// }

// pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
//     let mut total: u32 = 0;
//     let factors_iter = factors.iter();
    
//     factors_iter.for_each(|factor| total += multiples(*factor, limit));

//     total
// }
