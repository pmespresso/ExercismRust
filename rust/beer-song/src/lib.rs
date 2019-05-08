
pub fn verse(n: i8) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\n\
             Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\n\
			 Take it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\n\
			 Take one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        _ => format!("{0} bottles of beer on the wall, {0} bottles of beer.\n\
             Take one down and pass it around, {1} bottles of beer on the wall.\n", n, n - 1)
    }
}


// from a bottles to b bottles
pub fn sing(a: i8, b: i8) -> String {
    (b..a + 1)
        .map(|n| verse(n))
        .rev()
        .collect::<Vec<_>>() //https://stackoverflow.com/questions/40420608/cannot-infer-type-for-when-using-map-on-iter-in-rust
        .join("\n")
}



// Naive and poopy first attempt
// pub fn verse(n: i32) -> String {
//     let mut verse = String::with_capacity(n as usize);
//     let mut number_of_bottles = ();
//     let mut n;

//     if n == 0 {
//         number_of_bottles = ("No more", "Go to the store, buy some more ");
//     } else {
//         number_of_bottles = (n.to_string(), "Take one down, pass it around ")
//     }

//     verse.push_str(number_of_bottles.0);
//     verse.push_str(" ");
//     verse.push_str("bottles of beer on the wall, ");
//     verse.push_str(number_of_bottles.1);

//     n -= 1;

//     if n == 0 {
//         number_of_bottles = ("No more", "Go to the store, buy some more ");
//     } else {
//         number_of_bottles = (n.to_string(), "Take one down, pass it around ")
//     }

//     verse.push_str(number_of_bottles.0);
//     verse.push_str("bottles of beer.");

//     println!("{}", verse);

//     return verse;
// }

// pub fn sing(start: i32, end: i32) -> String {
//     unimplemented!("sing verses {} to {}, inclusive", start, end)
// }
