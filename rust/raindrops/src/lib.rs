pub fn raindrops(n: u32) -> String {
    // match (n % 3, n % 5, n % 7) {
    //     (0, 0, 0) => String::from("PlingPlangPlong"),
    //     (0, 0, _) => String::from("PlingPlang"),
    //     (_, 0, 0) => String::from("PlangPlong"),
    //     (0, _, 0) => String::from("PlingPlong"),
    //     (0, _, _) => String::from("Pling"),
    //     (_, 0, _) => String::from("Plang"),
    //     (_, _, 0) => String::from("Plong"),
    //     (_, _, _) => n.to_string(),
    // }

    let is_factor = |factor| n % factor == 0;
    let mut rez = String::new();

    if is_factor(3) { rez.push_str("Pling"); }
    if is_factor(5) { rez.push_str("Plang"); }
    if is_factor(7) { rez.push_str("Plong"); }

    if rez.is_empty() { rez = n.to_string(); }

    rez
}
