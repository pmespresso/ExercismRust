pub fn verse(n: i32) -> String {
    let mut verse = String::with_capacity(n as usize);
    let mut number_of_bottles = ();
    let mut n;

    if n == 0 {
        number_of_bottles = ("No more", "Go to the store, buy some more ");
    } else {
        number_of_bottles = (n.to_string(), "Take one down, pass it around ")
    }

    verse.push_str(number_of_bottles.0);
    verse.push_str(" ");
    verse.push_str("bottles of beer on the wall, ");
    verse.push_str(number_of_bottles.1);

    n -= 1;

    if n == 0 {
        number_of_bottles = ("No more", "Go to the store, buy some more ");
    } else {
        number_of_bottles = (n.to_string(), "Take one down, pass it around ")
    }

    verse.push_str(number_of_bottles.0);
    verse.push_str("bottles of beer.");

    println!("{}", verse);

    return verse;
}

pub fn sing(start: i32, end: i32) -> String {
    unimplemented!("sing verses {} to {}, inclusive", start, end)
}
