static PROGRAM: &'static str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Add,       // +
    Sub,       // -
    Right,     // >
    Left,      // <
    Read,      // ,
    Write,     // .
    BeginLoop, // [
    EndLoop,   // ]
}
use self::Token::*;

fn main() {
    let tokens = tokenize(PROGRAM);
    println!("//{:?}", tokens);

    let generated_code = generate(&tokens);
    println!("//Generated code:\n{}", generated_code);
}

fn tokenize(input:&str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        match c {
            '+' => tokens.push(Add),
            '-' => tokens.push(Sub),
            '>' => tokens.push(Right),
            '<' => tokens.push(Left),
            ',' => tokens.push(Read),
            '.' => tokens.push(Write),
            '[' => tokens.push(BeginLoop),
            ']' => tokens.push(EndLoop),
            _ => {}
        }
    }

    tokens
}

fn generate(tokens: &[Token]) -> String {
    let mut output = String::from(include_str!("preface.c"));

    for &token in tokens {
        match token {
            Add => {
                // Increment the value at the selected cell
                output.push_str("(*ptr)++;\n");
            }
            Sub => {
                // Decrement the value at the selected cell
                output.push_str("(*ptr)--;\n");
            }
            Right => {
                // Change our selected cell to the next to the right
                output.push_str("ptr++;\n");
            }
            Left => {
                // Change our selected cell to the next to the left
                output.push_str("ptr--;\n");
            }
            Read => {
                // Read a single character into the selected cell
                output.push_str("*ptr=getchar();\n");
            }
            Write => {
                // Print the character at the selected cell
                output.push_str("putchar(*ptr);\n");
            }
            BeginLoop => {
                // Begin a loop at the current cell
                output.push_str("while (*ptr) {\n");
            }
            EndLoop => {
                // Close a loop
                output.push_str("}\n");
            }
        }
    }

    output.push_str("}\n");
    output
}
