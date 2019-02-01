use std::process;

// Takes in a pre-defined word, matches it, and carries out its function
pub fn do_word(w: &String, stack: &mut Vec<i32>) {
    let stack_length = stack.len();

    match w.to_lowercase().as_ref() {
        "mod" => {
            if stack.len() >= 2 {
                stack[stack_length - 2] %= stack[stack_length - 1];
                stack.pop();
            }
        },
        "." => {
            if stack.len() > 0 {
                stack.pop();
            }
        },
        "dup" => {
            if stack.len() > 0 {
                stack.push(stack[stack_length - 1]);
            }
        },
        "clear" => {
            stack.clear();
        },
        ".s" => {
            for i in stack {
                print!("{} ", i);
            }
            println!();
        },
        _ => ()
    }
}

// Is this word proper or available?
pub fn is_word(w: &str) -> bool {
    let words = vec!["mod", ".", "dup", "clear", ".s"];

    for word in words {
        if w == word.to_uppercase() {
            return true;
        }
    }
    false
}