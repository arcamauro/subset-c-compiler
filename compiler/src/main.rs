use compiler::lexer::Lexer;
use compiler::parser::Parser;
use compiler::preprocessor::preprocess;

fn main() {
    let source = preprocess(
        r#"
        int main() {
            int a = 10; // a is equals to 10
            int b = 11; // b is equals to 11
            /*
             * Prints hello world ten times
             * Increments 'a' and decrements 'b'
             */
            for (int i = 0; i < 10; i++) {
                printf("Hello world\n");
                a += 2;
                b -= 1;
            }
            return 0;
        }
        "#,
    );

    let mut lexer = Lexer::new(&source);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.scan_token() {
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();
    println!("{:#?}", program);
}
