static PROGRAM: &'static str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

mod tokenizer;

fn main() {
    let tokens = tokenizer::tokenize(PROGRAM);
    println!("//{:?}", tokens);

    let generated_code = tokenizer::generate(&tokens);
    println!("//Generated code:\n{}", generated_code);
}
