// What are the prime factors of 60?

// Our first divisor is 2. 2 goes into 60, leaving 30.
// 2 goes into 30, leaving 15.
// 2 doesn't go cleanly into 15. So let's move on to our next divisor, 3.
// 3 goes cleanly into 15, leaving 5.
// 3 does not go cleanly into 5. The next possible factor is 4.
// 4 does not go cleanly into 5. The next possible factor is 5.
// 5 does go cleanly into 5.
// We're left only with 1, so now, we're done.

pub fn factors(mut n: u64) -> Vec<u64> {
    let mut prime_factors: Vec<u64> = Vec::new();
    let mut divisors = 2..;

    while n > 1 {
        let divisor = divisors.next().unwrap();

        while n % divisor == 0 {
             n /= divisor;
             prime_factors.push(divisor)
        }
    }

    prime_factors
}
