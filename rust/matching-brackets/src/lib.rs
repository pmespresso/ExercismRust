// Algorithm:
// 1) Declare a character stack S.
// 2) Now traverse the expression string exp.
//     a) If the current character is a starting bracket (‘(‘ or ‘{‘ or ‘[‘) then push it to stack.
//     b) If the current character is a closing bracket (‘)’ or ‘}’ or ‘]’) then pop from stack 
//            and if the popped character is the matching starting bracket then fine else parenthesis are not balanced.
// 3) After complete traversal, if there is some starting bracket left in stack then “not balanced”

// Attempt 2
pub fn brackets_are_balanced(string: &str) -> bool {
  let mut brack: Vec<char> = Vec::new();

  for c in string.chars() {
    match c {
      '(' | '{' | '[' => brack.push(c),
      ')' => if brack.pop() != Some('(') { return false; },
      '}' => if brack.pop() != Some('{') { return false; },
      ']' => if brack.pop() != Some('[') { return false; },
      _ => ()
    }
  }

  brack.is_empty()
}














// Attempt # 1
// use std::collections::HashMap;
// pub fn brackets_are_balanced(string: &str) -> bool {
//   let mut stack: Vec<char> = Vec::new();
//   let mut matching_brackets = HashMap::new();
//   matching_brackets.insert('(', ')');
//   matching_brackets.insert('{', '}');
//   matching_brackets.insert('[', ']');
//   matching_brackets.insert(']', '[');
//   matching_brackets.insert('}', '{');
//   matching_brackets.insert(')', '(');

//   string.chars().map(|c| match c {
//     c if c == '(' || c == '{' || c == '[' => {
//       stack.push(c);
//       return true
//     },
//     c if c == ')' || c == '}' || c == ']' => match stack.pop().unwrap() {
//       popped if matching_brackets[&popped] != c => return false,
//       _ => return true,
//     },
//     _ => return false
//   });

//   match stack.len() {
//     0 => true,
//     _ => false
//   }
// }
