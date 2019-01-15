#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Add,       // +
    Sub,       // -
    Right,     // >
    Left,      // <
    Read,      // ,
    Write,     // .
    BeginLoop, // [
    EndLoop,   // ]
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        match c {
            '+' => tokens.push(Token::Add),
            '-' => tokens.push(Token::Sub),
            '>' => tokens.push(Token::Right),
            '<' => tokens.push(Token::Left),
            ',' => tokens.push(Token::Read),
            '.' => tokens.push(Token::Write),
            '[' => tokens.push(Token::BeginLoop),
            ']' => tokens.push(Token::EndLoop),
            _ => {}
        }
    }

    tokens
}

pub fn generate(tokens: &[Token]) -> String {
    let mut output = String::from(include_str!("preface.c"));

    for &token in tokens {
        match token {
            Token::Add => {
                // Increment the value at the selected cell
                output.push_str("(*ptr)++;\n");
            }
            Token::Sub => {
                // Decrement the value at the selected cell
                output.push_str("(*ptr)--;\n");
            }
            Token::Right => {
                // Change our selected cell to the next to the right
                output.push_str("ptr++;\n");
            }
            Token::Left => {
                // Change our selected cell to the next to the left
                output.push_str("ptr--;\n");
            }
            Token::Read => {
                // Read a single character into the selected cell
                output.push_str("*ptr=getchar();\n");
            }
            Token::Write => {
                // Print the character at the selected cell
                output.push_str("putchar(*ptr);\n");
            }
            Token::BeginLoop => {
                // Begin a loop at the current cell
                output.push_str("while (*ptr) {\n");
            }
            Token::EndLoop => {
                // Close a loop
                output.push_str("}\n");
            }
        }
    }

    output.push_str("}\n");
    output
}
