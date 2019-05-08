/*
    Bob is a lackadaisical teenager. In conversation, his responses are very limited.

    Bob answers 'Sure.' if you ask him a question.

    He answers 'Whoa, chill out!' if you yell at him.

    He answers 'Calm down, I know what I'm doing!' if you yell a question at him.

    He says 'Fine. Be that way!' if you address him without actually saying anything.

    He answers 'Whatever.' to anything else.

    Bob's conversational partner is a purist when it comes to written communication and always follows
    normal rules regarding sentence punctuation in English.
*/

fn is_yelling(message: &str) -> bool {
    let hasLetters: bool = message.chars().filter(|x| x.is_alphabetic()).count() > 0;
    message.to_uppercase() == message && hasLetters // is all caps?
}

pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.len() == 0 => "Fine. Be that way!",
        m if m.ends_with("?") && !is_yelling(m) => "Sure.",
        m if m.ends_with("?") && is_yelling(m) => "Calm down, I know what I'm doing!",
        m if is_yelling(m) => "Whoa, chill out!",
        _ => "Whatever."
    }
}
