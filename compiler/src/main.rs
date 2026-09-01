use compiler::lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new(
        r#"
        #include <stdio.h>
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

    while let Some(token) = lexer.scan_token() {
        println!("{:?}", token);
    }
}
