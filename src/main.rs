use std::io;

fn main() {
    println!("enter the phrase: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("please enter a valid input");
    println!(
        "{}",
        if is_balanced(input) {
            "Nice"
        } else {
            "what the hell"
        }
    )
}

fn is_balanced(input: String) -> bool {
    let mut stack = Vec::new();
    for char in input.chars() {
        if char == '(' {
            stack.push(char);
            continue;
        }
        if char == ')' {
            stack.pop();
        }
    }
    return if stack.len() == 0 { true } else { false };
}
