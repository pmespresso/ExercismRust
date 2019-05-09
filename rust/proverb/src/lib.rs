// For want of a horseshoe nail, a kingdom was lost, or so the saying goes.

// Given a list of inputs, generate the relevant proverb. For example, given the list ["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"], you will output the full text of this proverbial rhyme:

// For want of a nail the shoe was lost.
// For want of a shoe the horse was lost.
// For want of a horse the rider was lost.
// For want of a rider the message was lost.
// For want of a message the battle was lost.
// For want of a battle the kingdom was lost.
// And all for the want of a nail.
pub fn build_proverb(list: Vec<&str>) -> String {
    let mut wordlist_iter = list.iter();
    let first_word = match wordlist_iter.next() {
        Some(word) => word,
        _ => return String::new()
    };

    let mut want = first_word;
    let mut solution = vec!();

    for lost in wordlist_iter {
        solution.push(
            format!("For want of a {0} the {1} was lost.", want, lost)
        );

        want = lost;
    }

    solution.push(format!("And all for the want of a {0}.", first_word));

    solution.join("\n")
}




// Naive first pass
// pub fn build_proverb(list: &[&str]) -> String {
//     let list_len = list.len();
//     let first_word = list[0];
//     let solution: Vec<&str> = Vec::with_capacity(list_len);

//     (0..list_len)
//         .map(|i| solution.push(format!("For want of a {0} the {1} was lost.\n", list[i], list[i - 1])).to_string());
    
//     solution.push(&format!("And all for the want of a {}.", first_word));

//     let joined = solution.join("");
//     joined
// }
