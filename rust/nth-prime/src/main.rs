extern crate itertools;

use itertools::Itertools;

pub fn nth(x: u32) -> Result<u64, String> {
    match x {
        1 => Ok(2),
        2 => Ok(3),
        3 => Ok(5),
        4 => Ok(7),
        5 => Ok(11),
        x if x <= 0 => Err("invalid input".into()),
        x if x < 7 => Ok(trial_div(x)),
        x => Ok(sieve(x)),
    }
}

fn trial_div(x: u32) -> u64 {
    let mut primes: Vec<u64> = Vec::with_capacity(x as usize);

    primes.push(2);

    if x > 1 {
        primes.push(3);
    }

    let mut next_checked = *primes.last().unwrap() + 2;

    while primes.len() < x as usize {
        if primes.iter().all(|&i| next_checked % i != 0) {
            primes.push(next_checked)
        }
        next_checked += 2;
    }
    *primes.last().unwrap()
}

fn sieve(x: u32) -> u64 {
    let upper_limit = {
        let x = x as f64;
        (x * ((x * x.ln()).ln())).floor() as usize
    };
    let mut prime_indices = vec![true; upper_limit];

    for p in prime_indices.iter_mut().skip(4).step(2) {
        *p = false;
    }
    for i in (3..upper_limit).step(2) {
        if prime_indices[i] {
            for p in prime_indices.iter_mut().skip(i * i).step(2 * i) {
                *p = false;
            }
        }
    }

    let prime_x = prime_indices.into_iter()
        .enumerate() // keep track of original indices
        .skip(2) // start at 2
        .filter(|&(_, p)| p) // primes only
        .map(|(i, _)| i) // discard booleans
        .nth((x - 1) as usize) // take the xth prime (1-indexed)
        .unwrap_or(1); // default to 1

    prime_x as u64
}

fn main(x: u32) -> u64 {
    let upper_limit = {
        let x = x as f64;
        (x * ((x * x.ln()).ln())).floor() as usize
    };
}
