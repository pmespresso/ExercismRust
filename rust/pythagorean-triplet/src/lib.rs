/*
    a^2 + b^2 = c^2

    can be written in terms of m and n such that

    a = m^2 - n^2
    b = 2 * m * n
    c = m^2 + n^2

    so then

    a^2 = m^4 + n^4 - 2 + m^2 * n^2
    b^2 = 4* m^2 * n^2
    c^2 = m^4 + n^4 + 2 + m^2 * n^2
*/

pub fn find() -> Option<u32> {
    let mut a: u32;
    let mut b: u32;
    let mut c: u32;

    let mut m: u32 = 2;

    loop {
        let mut n: u32 = 1;

        while n < m {
            a = m * m - n * n;
            b = 2 * m * n;
            c = m * m + n * n;
            n = n + 1;

            if a + b + c == 1000 {
                println!("{} {} {} {} {}", a, b, c, m, n);
                return Some(a * b * c);
            }
        }
        m = m + 1;
    }
}
